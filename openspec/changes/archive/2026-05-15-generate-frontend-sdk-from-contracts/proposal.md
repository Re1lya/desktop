## Why

The repository already generates TypeScript DTO types from `ora-contracts`, but frontend code still has to hand-maintain HTTP methods, route paths, and request assembly details that actually live in the Rust server. That duplication is now a drag on frontend work because the API surface is growing and the current export flow cannot produce a safe, typed SDK from the existing contracts alone.

## What Changes

- Add a repo-owned frontend SDK generation flow that emits endpoint metadata, a typed client, and transport helpers from Rust-side contract definitions instead of relying on manually maintained frontend API paths.
- Extend `ora-contracts` with endpoint manifest definitions that capture operation name, HTTP method, path template, request type, response type, path parameter mapping, and JSON body behavior for the exported HTTP surface.
- Generate a runtime-agnostic TypeScript client in `packages/contracts` that builds URLs, serializes JSON bodies, and returns typed responses through an injected transport interface.
- Expose a browser `fetch` transport entrypoint that resolves requests against a server base URL and normalizes the web server's shared error envelope.
- Replace the current test-driven TypeScript contract export path with a dedicated `cargo xtask` workflow that becomes the canonical contract and SDK generator.

## Capabilities

### New Capabilities
- `frontend-contract-sdk`: Define the generated endpoint manifest, typed TypeScript SDK surface, transport contract, and browser `fetch` integration exported from `packages/contracts`.

### Modified Capabilities
- `app-contracts`: Expand the contract export surface so `ora-contracts` remains the single Rust source of truth for frontend DTOs and HTTP endpoint metadata used to generate the SDK.

## Impact

- Affected code: `crates/contracts`, a new workspace `xtask` crate, `packages/contracts`, and supporting workspace task wiring.
- Affected APIs: public `ora-contracts` endpoint metadata exports, generated `@ora/contracts` SDK entrypoints, and the new `@ora/contracts/fetch` transport entrypoint.
- Dependencies: expected workspace changes for generator plumbing, with no immediate need to adopt a full OpenAPI toolchain.
- Systems: removes manual frontend route duplication, keeps server route knowledge aligned with Rust contracts, and establishes a canonical contract export workflow for future frontend integration work.
