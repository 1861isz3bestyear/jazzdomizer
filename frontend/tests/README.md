# Frontend tests

This directory contains dependency-free tests for the plain JavaScript frontend. They use Node's built-in `node:test` runner and the CommonJS exports already exposed by the frontend files.

- `api.unit.test.js` contains unit tests for `ApiClient`: URL construction, request headers, search normalization, response unwrapping, and error handling.
- `song_list.module.test.js` contains module-level tests for song mapping, catalog behavior, presenter HTML escaping/rendering, and the song list view flow.
- `app.integration.test.js` contains integration tests for the app router with a minimal DOM stub, covering the songs route, unknown route handling, and startup routing.

`practice_mode.js` is intentionally not covered here.

Run from the repository root with:

```sh
node --test frontend/tests/*.test.js
```
