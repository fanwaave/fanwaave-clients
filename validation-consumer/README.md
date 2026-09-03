# lib-core validation consumer

These adapters are part of the client SDK surface. They import the public validation packages from `fanwaave-lib-core`; they do not copy schemas and they cannot import the server-only packages.

Each request boundary validates `RequestMeta` before transport. Problem responses are validated as `ProblemDetails` after transport. Route-specific payload validators will be selected by the `ORESoftware/api-docs` operation ID once a binding is present in `fanwaave-interfaces/validation/route-bindings.v1.json`.

The CI workflow checks out the exact lib-core commit used by this change and compiles/tests TypeScript, Rust, Go, and Gleam consumers. Updating the lib-core revision requires the same review as updating the public interface authorities.
