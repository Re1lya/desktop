use crate::app_state::AppState;
use crate::config::RuntimeConfig;
use crate::error::WebBootstrapError;
use crate::service::{FileSystemApi, ProjectWorkContextApi};
use ora_application::Clock;
use ora_backend::{Backend, BackendBootstrapError, BackendPaths};
use std::path::Path;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Builds the application state used by the web runtime from SQLite-backed dependencies.
pub fn build_app_state(runtime_config: &RuntimeConfig) -> Result<AppState, WebBootstrapError> {
    let backend = build_backend(
        runtime_config.database().path(),
        runtime_config.worktree().root(),
        runtime_config.file_system().home_directory(),
    )?;
    let pool = backend.repository_pool();
    let clock = SystemClock;

    Ok(AppState::new(
        backend,
        Arc::new(FileSystemApi::new(
            runtime_config.file_system().home_directory().to_path_buf(),
        )),
        Arc::new(ProjectWorkContextApi::new(pool, clock)),
    ))
}

/// Builds application state for tests from explicit filesystem paths.
#[cfg(test)]
pub(crate) fn build_app_state_for_database(
    database_path: &Path,
    project_root: &Path,
    work_dir: &Path,
) -> Result<AppState, WebBootstrapError> {
    let backend = build_backend(
        database_path,
        work_dir,
        project_root.parent().unwrap_or(project_root),
    )?;
    let pool = backend.repository_pool();
    let clock = SystemClock;

    Ok(AppState::new(
        backend,
        Arc::new(FileSystemApi::new(
            project_root.parent().unwrap_or(project_root).to_path_buf(),
        )),
        Arc::new(ProjectWorkContextApi::new(pool, clock)),
    ))
}

/// Opens the shared backend while preserving the server's existing bootstrap error variants.
fn build_backend(
    database_path: &Path,
    worktree_root: &Path,
    home_directory: &Path,
) -> Result<Backend, WebBootstrapError> {
    Backend::open(BackendPaths {
        database_path: database_path.to_path_buf(),
        worktree_root: worktree_root.to_path_buf(),
        home_directory: home_directory.to_path_buf(),
    })
    .map_err(web_backend_bootstrap_error)
}

/// Maps shared backend bootstrap failures into the stable Web process error surface.
fn web_backend_bootstrap_error(error: BackendBootstrapError) -> WebBootstrapError {
    match error {
        BackendBootstrapError::DirectoryCreate { source, .. } => {
            WebBootstrapError::DataDirectoryCreate(source)
        }
        BackendBootstrapError::Database(source) => WebBootstrapError::DatabaseBootstrap(source),
        BackendBootstrapError::AgentRuntime(source) => {
            WebBootstrapError::BackendRuntimeBootstrap(source)
        }
    }
}

/// Reads the current wall-clock time for audit fields in the runtime.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SystemClock;

impl Clock for SystemClock {
    /// Returns the current Unix timestamp in milliseconds for handler audit fields.
    fn now_timestamp_millis(&self) -> i64 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_millis() as i64,
            Err(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{build_app_state, build_app_state_for_database};
    use crate::config::RuntimeConfig;
    use crate::error::WebBootstrapError;
    use ora_application::{ProjectRepository, ProjectWorkContextRepository};
    use ora_db::{
        DatabaseBootstrapper, DatabaseLocation, SqliteProjectRepository,
        SqliteProjectWorkContextRepository, default_migration_catalog,
    };
    use ora_domain::ProjectWorkContextSurface;
    use pretty_assertions::assert_eq;
    use std::path::Path;
    use tempfile::TempDir;

    /// Verifies bootstrap fails cleanly when the configured database path points to a directory.
    #[test]
    fn rejects_directory_database_paths() {
        let temp_dir = TempDir::new().unwrap();
        let error = match build_app_state_for_database(
            temp_dir.path(),
            temp_dir.path(),
            &temp_dir.path().join("worktrees"),
        ) {
            Ok(_) => panic!("expected directory database path to fail"),
            Err(error) => error,
        };

        assert!(matches!(error, WebBootstrapError::DatabaseBootstrap(_)));
    }

    /// Verifies runtime bootstrap becomes usable without creating a project or synthetic context.
    #[test]
    fn starts_with_an_empty_project_catalog() {
        let temp_dir = TempDir::new().unwrap();
        let data_dir = temp_dir.path().join("empty-bootstrap");
        let runtime_config = runtime_config(&data_dir);
        let database_path = data_dir.join("ora.sqlite3");

        build_app_state(&runtime_config)
            .unwrap_or_else(|error| panic!("expected runtime bootstrap to succeed: {error}"));

        let repository = bootstrapped_project_repository(&database_path);
        let context_repository = bootstrapped_project_work_context_repository(&database_path);

        assert_eq!(repository.list_projects().unwrap(), Vec::new());
        assert_eq!(
            context_repository
                .find_project_work_context(ProjectWorkContextSurface::Web, "main")
                .unwrap(),
            None
        );
    }

    /// Builds one runtime configuration without mutating process environment during tests.
    fn runtime_config(data_dir: &Path) -> RuntimeConfig {
        RuntimeConfig::from_reader(|key| match key {
            "ORA_DATA_DIR" => Some(data_dir.to_string_lossy().to_string()),
            "HOME" => Some(data_dir.to_string_lossy().to_string()),
            _ => None,
        })
        .unwrap_or_else(|error| panic!("expected runtime configuration to load: {error}"))
    }

    /// Opens the test database so bootstrap assertions can inspect persisted project state.
    fn bootstrapped_project_repository(database_path: &Path) -> SqliteProjectRepository {
        let pool = DatabaseBootstrapper::system()
            .bootstrap_repository_pool(
                &DatabaseLocation::path(database_path),
                &default_migration_catalog().unwrap(),
            )
            .unwrap_or_else(|error| {
                panic!("expected repository pool bootstrap to succeed: {error}")
            });

        SqliteProjectRepository::new(pool)
    }

    /// Opens the test database so bootstrap assertions can inspect persisted project work context state.
    fn bootstrapped_project_work_context_repository(
        database_path: &Path,
    ) -> SqliteProjectWorkContextRepository {
        let pool = DatabaseBootstrapper::system()
            .bootstrap_repository_pool(
                &DatabaseLocation::path(database_path),
                &default_migration_catalog().unwrap(),
            )
            .unwrap_or_else(|error| {
                panic!("expected repository pool bootstrap to succeed: {error}")
            });

        SqliteProjectWorkContextRepository::new(pool)
    }
}
