# SpacetimeDB Rust SDK (Client Notes for Ikaria)

Source: https://spacetimedb.com/docs/sdks/rust

## Purpose
This document summarizes the SpacetimeDB Rust client SDK for the Ikaria client and highlights the pieces we should use in Bevy.

## Setup and generated bindings
- Add SDK dependency in a standalone client project:
  - `cargo add spacetimedb_sdk`
- Generate module bindings with the CLI:
  - `spacetime generate --lang rust --out-dir src/module_bindings --project-path PATH-TO-MODULE`
- Generic examples in SpacetimeDB docs use `mod module_bindings;`.
- In this repository, generated Rust bindings are already managed at workspace level (`task sdk-rust`) and consumed via `ikaria-types`.
- Current client target module for development is `ikariadb-dev`.

## Connection model
- Main connection type: `module_bindings::DbConnection`.
- Build with `DbConnection::builder()` then configure:
  - `with_uri(...)`
  - `with_module_name(...)`
  - `with_confirmed_reads(...)` (durability vs latency tradeoff)
  - `with_token(...)` (reuse saved auth token, or anonymous if omitted)
  - `on_connect(...)`, `on_disconnect(...)`, then `build()`
- Connection callbacks:
  - `on_connect` provides `Identity` + private token (save token for future login).
  - `on_disconnect` handles disconnect/error closure.
  - Docs currently note a bug where `on_connect_error` may not fire; rely on `on_disconnect` handling as well.

## You must advance the connection
If the connection is not advanced, no network work or callbacks happen.

Options:
- `run_threaded()` for a background thread.
- `run_async()` in async runtime.
- `frame_tick()` for game loops (chosen integration point for Ikaria Bevy update flow).
- Ikaria policy: run `frame_tick()` only after the first screen, once connection has been established.

## DbContext and callback contexts
`DbConnection` and callback contexts implement `DbContext`:
- `DbConnection`
- `EventContext` (row callbacks)
- `ReducerEventContext` (reducer callbacks)
- `SubscriptionEventContext` (subscription lifecycle callbacks)
- `ErrorContext` (error callbacks)

Useful `DbContext` methods:
- `db()` / `reducers()` (trait-generic access)
- `disconnect()`
- `identity()` / `try_identity()`
- `connection_id()`
- `is_active()`

## Subscriptions
Use `ctx.subscription_builder()` to subscribe to SQL queries.

Important pieces:
- `on_applied(...)` runs when rows are in local cache.
- `on_error(...)` runs for subscription failures (often invalid query).
- `subscribe(queries)` returns `SubscriptionHandle`.
- `subscribe_to_all_tables()` exists and is the current Ikaria choice, with view/state systems controlling what players can see.

`SubscriptionHandle` lifecycle:
- `is_active()`, `is_ended()`
- `unsubscribe()`
- `unsubscribe_then(...)` for cleanup after rows are removed

## Client cache API (tables)
Subscribed rows are accessed through `ctx.db` / `ctx.db()`.

Per-table capabilities:
- `count()`
- `iter()`
- `on_insert(...)`
- `on_delete(...)`
- `on_update(...)` (for tables with primary key via `TableWithPrimaryKey`)

Indexes:
- Unique/primary-key index handles support `.find(...)`.
- Non-unique BTree index access is not supported in Rust SDK.

## Reducers
`ctx.reducers` / `ctx.reducers()` exposes generated reducer methods.

For each reducer you typically get:
- invoke method (snake_case reducer name),
- `on_<reducer>(...)` callback registration,
- `remove_on_<reducer>(...)` callback removal.

Reducer metadata is exposed via `ReducerEvent`:
- timestamp, status (`Committed` / `Failed` / `OutOfEnergy`),
- caller identity/connection,
- optional energy consumed,
- reducer + args payload.

## Identity and auth notes
- `Identity`: stable public user identity.
- `ConnectionId`: specific connection instance for that identity.
- For token-based re-login:
  1. Read saved token from file and pass to `with_token(...)`.
  2. On successful `on_connect`, persist latest token returned by callback.

## Bevy integration guidance for Ikaria
- Prefer driving networking with `frame_tick()` in a state-gated update system.
- Register row/reducer callbacks close to feature setup and clean them up on state exit.
- Keep `SubscriptionHandle`s so view transitions can unsubscribe cleanly.
- Route callback side effects into Bevy resources/events to keep screen plugins decoupled.
