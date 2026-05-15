## 1. Add Rust-side endpoint metadata

- [x] 1.1 Add an endpoint manifest module to `crates/contracts` that defines the exported CRUD operations with HTTP method, full path template, request type, response type, path parameter fields, and JSON body behavior.
- [x] 1.2 Re-export the endpoint metadata from `crates/contracts/src/lib.rs` and add focused Rust tests that verify the manifest matches the current contract surface and request-splitting rules.
- [x] 1.3 Align the manifest entries with the current web-server route table so project CRUD paths and operation names remain consistent between `ora-contracts` and `apps/web/server`.

## 2. Introduce the canonical contract export workflow

- [x] 2.1 Add a workspace `xtask` crate with a `cargo xtask export-contracts` entrypoint that owns contract and SDK generation.
- [x] 2.2 Move the existing TypeScript DTO export into that xtask workflow and extend it to emit the generated endpoint manifest and typed client artifacts into `packages/contracts`.
- [x] 2.3 Update workspace task wiring so any convenience command delegates to `cargo xtask export-contracts` instead of relying on test-driven generation.

## 3. Generate the TypeScript SDK surface

- [x] 3.1 Add generated SDK output files in `packages/contracts` for endpoint metadata, shared request-building helpers, transport interfaces, and typed client construction.
- [x] 3.2 Implement generator logic that interpolates declared path parameters, serializes JSON bodies from the remaining request fields, and preserves typed success responses for each exported operation.
- [x] 3.3 Add package-level tests or fixture-based verification that cover representative generated operations, including endpoints that combine path parameters with JSON body payloads.

## 4. Add browser fetch transport support

- [x] 4.1 Add the `@ora/contracts/fetch` entrypoint with a browser `fetch` transport that resolves endpoint paths against a server `baseUrl` and treats an empty base as the current origin.
- [x] 4.2 Implement shared HTTP error decoding for the web server's JSON error envelope so SDK callers receive normalized transport errors instead of raw response handling.
- [x] 4.3 Add focused tests for base URL resolution and structured error decoding behavior in the fetch transport layer.

## 5. Document and verify the generated workflow

- [x] 5.1 Update affected `docs/` content to describe the new contract export workflow and generated SDK package shape where applicable.
- [x] 5.2 Run `cargo fmt --all` and `task test`, then resolve any formatting, generation, compile, or test regressions introduced by the new contract SDK workflow.
