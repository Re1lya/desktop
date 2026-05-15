## Context

`ora-contracts` already owns the DTO layer that gets exported to `packages/contracts`, but it does not encode the HTTP information needed to build a safe frontend SDK. That information still lives in `apps/web/server/src/routes.rs`, which means frontend callers must duplicate method names, route paths, path parameter handling, request body splitting, and shared transport error parsing.

The change crosses Rust contracts, TypeScript package output, and workspace generation workflow, so it benefits from a design pass before implementation. The existing direction in `docs/BRAINSTORM.md` favors a small repo-local manifest over introducing OpenAPI tooling, and it keeps Rust as the source of truth without coupling `ora-contracts` to the web-server crate.

## Goals / Non-Goals

**Goals:**

- Define endpoint metadata in `ora-contracts` so Rust owns both DTOs and frontend-facing HTTP shape.
- Generate a TypeScript endpoint manifest and typed client in `packages/contracts` from Rust-owned metadata.
- Keep the generated client runtime-agnostic by delegating actual HTTP execution to an injected transport trait or interface.
- Expose a browser-friendly `fetch` transport package entrypoint that resolves against a server base URL and decodes the shared web-server error envelope.
- Replace the current test-based export flow with a dedicated `cargo xtask export-contracts` workflow that becomes the canonical generator.

**Non-Goals:**

- Adopting a full OpenAPI schema, code generator, or annotation stack in this change.
- Moving route registration authority out of `apps/web/server` or making the server depend on generated TypeScript artifacts.
- Generating adapter-specific clients for Tauri, Node, or other runtimes beyond the browser `fetch` helper.
- Solving advanced transport concerns such as retries, auth policies, or streaming APIs in the first SDK slice.

## Decisions

### Add an explicit endpoint manifest module in `ora-contracts`

`ora-contracts` will gain a Rust-side endpoint description module that defines one manifest entry per exported HTTP operation. Each entry will include the operation name, HTTP method, full path template, request contract type, response contract type, path parameter field names, and whether the request has a JSON body.

Why:
- The current DTO export proves that `ora-contracts` is already the right place for frontend-facing contract ownership.
- An explicit manifest avoids brittle inference from DTO naming or server route parsing.
- Keeping the manifest in Rust lets the server and contract generator share the same vocabulary without creating a reverse dependency from contracts to the web crate.

Alternative considered:
- Infer endpoint metadata directly from `apps/web/server/src/routes.rs`.
  Rejected because it would make the generator depend on adapter wiring and would keep route knowledge outside the contract crate.

### Generate a runtime-agnostic client plus a separate `fetch` transport

The generated `@ora/contracts` client will only know how to turn typed request objects into a normalized transport request shape and how to deserialize typed success responses. A sibling `@ora/contracts/fetch` entrypoint will provide the default browser transport implementation and shared HTTP error normalization against the server's JSON error envelope.

Why:
- An injected transport keeps the client usable from browser, tests, and future runtimes without forking code generation.
- Separating the browser transport avoids hard-coding `fetch` into the core SDK while still giving frontend code a turnkey default.
- This matches the repo's broader preference for dependency injection and transport-agnostic business boundaries.

Alternative considered:
- Generate a single browser-only client that directly calls `fetch`.
  Rejected because it would make testing harder and would prematurely lock the SDK to one runtime.

### Preserve request-shape ownership in contract DTOs and only annotate transport splitting

Request DTOs will remain the public typed input shape for the client, while the endpoint manifest will specify which fields belong in the path and whether the remaining payload should be serialized as JSON. The generator will not require separate path-only and body-only DTO variants.

Why:
- Existing handlers and DTOs already model the semantic request shape, and splitting them into transport fragments would add churn across application and server layers.
- A small amount of manifest metadata is enough to teach the generator how to assemble requests without introducing parallel request-type families.

Alternative considered:
- Redesign contracts so each endpoint exposes separate path and body structs.
  Rejected because it would spread transport concerns into the contract surface and increase API verbosity for little gain.

### Make `cargo xtask export-contracts` the canonical export pipeline

The workspace will add an `xtask` crate that drives DTO export, endpoint manifest export, and typed client generation into `packages/contracts`. Task wrappers may still call into it, but test execution will no longer be the primary generation mechanism.

Why:
- Code generation should be an intentional build step rather than a side effect of running tests.
- A dedicated task gives the repository one place to orchestrate artifact cleanup, output layout, and future export expansion.
- It reduces confusion for contributors by making contract generation explicit and repeatable.

Alternative considered:
- Keep the current export test and append more generated files to it.
  Rejected because it blurs verification with generation and will become harder to maintain as outputs grow.

## Risks / Trade-offs

- [The endpoint manifest adds another contract artifact to maintain] -> Mitigation: keep the manifest intentionally small and colocated with DTO ownership so route metadata only has one Rust home.
- [Server routes and contract manifests could drift if updated separately] -> Mitigation: align naming and path templates directly with the current route table and add generator or integration checks where practical during implementation.
- [The first generated client may not cover unusual request patterns] -> Mitigation: scope the initial manifest to the current CRUD HTTP surface and encode only the transport distinctions the repo already uses.
- [Changing the export workflow could disrupt existing contributor habits] -> Mitigation: preserve `Taskfile.yml` as a thin wrapper if needed, but document `cargo xtask export-contracts` as the source of truth.

## Migration Plan

1. Introduce the endpoint manifest in `ora-contracts` alongside the current DTO exports.
2. Add the `xtask` generator and make it emit the existing DTO outputs plus the new SDK artifacts into `packages/contracts`.
3. Add the generated client and `fetch` transport entrypoints without removing existing direct DTO consumption.
4. Update package documentation and task wrappers to point contributors at `cargo xtask export-contracts`.
5. Migrate frontend call sites to the generated SDK in follow-up implementation work.

## Open Questions

- None for proposal readiness. The current brainstorming direction is specific enough to implement without another design round.
