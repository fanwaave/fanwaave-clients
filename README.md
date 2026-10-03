# fanwaave-clients

Polyglot Fanwaave SDKs live under `clients/`. Rust, TypeScript, and Dart are first-class and modular; the broader language matrix remains supported by the shared contract/generation pipeline.

## Contract and RPC authority

- `fanwaave/fanwaave-interfaces` owns shared public contracts. TypeSpec and JSON Schema remain independent peer authorities.
- `fanwaave/fanwaave-pub-lib-core` owns public client-side composition. This repository must not depend on server-only `fanwaave-lib-core`.
- `ORESoftware/typespec-json-schema-validator` produces parity-approved, digest-bound Contract IR.
- `ORESoftware/api-docs` supplies the shared operation/RPC runtime, documentation projections, and polyglot generation machinery.
- `ores-stack` owns RPC operation discovery and client projection. The public client projection is declared in `rpc/public-projection.v1.json`; generated RPC sources must come from an admitted API-server operation index and must not be edited by hand.
- `ORESoftware/ores-wit` is the WIT/component binding lane. It is intentionally not a hard dependency until its reproducible `Cargo.lock` promotion blocker is closed; the exact reviewed source revision is retained in `toolchain/ores-toolchain.v1.json`.

The canonical ORES Stack identity is `ores-stack/ores-stack-cli`. While that repository is still completing its migration/promotion checklist, the last complete `ORESoftware/ores-stack` source is recorded as a temporary executable generator. Promotion must preserve operation-index, Contract-IR, and generated-manifest digests before the fallback is removed.

## Public RPCs

Only operations authored with regular scope and explicit `browser` audience are eligible for this repository. Admin operations and server-only operations cannot be widened by the client generator. The initial public operation is `fanwaave.health.get`; its API-server operation authority is landed separately before generated sources are promoted.

Run the repository-local policy check with:

```sh
node scripts/check-ores-toolchain.mjs
```
