# Backend integration tests

This directory contains integration tests for the public `jazzdomizer_backend` crate API.

- `api_integration.rs` exercises the public Axum router through HTTP requests. It covers `/api/health` and CORS behavior without connecting to Postgres by using a lazy `PgPool`.
- `music_integration.rs` covers cross-module music behavior: database DTO conversion, key parsing, harmony parsing, chord parsing, transposition, enharmonic normalization, and display formatting.

The API song catalog routes currently depend on a real `PgPool` through the public `create_api_router` function. Because `backend/src` was left unchanged, these tests avoid database-backed routes and focus on public behavior that is deterministic without external services.

Run from `backend/` with:

```sh
cargo test --tests
```
