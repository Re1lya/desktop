mod project;
mod session;
mod task;
mod worktree;

pub use project::{
    CreateProjectRequest, CreateProjectResponse, DeleteProjectRequest, DeleteProjectResponse,
    GetProjectRequest, GetProjectResponse, ListProjectsRequest, ListProjectsResponse, Project,
    UpdateProjectRequest, UpdateProjectResponse,
};
pub use session::{
    CreateSessionRequest, CreateSessionResponse, DeleteSessionRequest, DeleteSessionResponse,
    GetSessionRequest, GetSessionResponse, ListSessionsRequest, ListSessionsResponse, Session,
    SessionStatus, UpdateSessionRequest, UpdateSessionResponse,
};
pub use task::{
    CreateTaskRequest, CreateTaskResponse, DeleteTaskRequest, DeleteTaskResponse, GetTaskRequest,
    GetTaskResponse, ListTasksRequest, ListTasksResponse, Task, TaskStatus, UpdateTaskRequest,
    UpdateTaskResponse,
};
pub use worktree::{
    CreateWorktreeRequest, CreateWorktreeResponse, DeleteWorktreeRequest, DeleteWorktreeResponse,
    GetWorktreeRequest, GetWorktreeResponse, ListWorktreesRequest, ListWorktreesResponse,
    UpdateWorktreeRequest, UpdateWorktreeResponse, Worktree, WorktreeActivity,
};

#[cfg(test)]
mod export_tests {
    use super::{
        CreateProjectRequest, CreateSessionRequest, CreateTaskRequest, CreateWorktreeRequest,
    };
    use ts_rs::{Config, TS};

    /// Exports every contract family into the shared TypeScript package for frontend consumers.
    #[test]
    fn exports_typescript_bindings() {
        let config = Config::from_env();

        CreateProjectRequest::export_all(&config).unwrap();
        CreateSessionRequest::export_all(&config).unwrap();
        CreateTaskRequest::export_all(&config).unwrap();
        CreateWorktreeRequest::export_all(&config).unwrap();
    }
}
