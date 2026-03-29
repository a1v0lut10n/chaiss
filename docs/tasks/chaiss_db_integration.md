# Implementation Plan: Asynchronous Database Dispatch

This document maps out the required GUI scaffolding to seamlessly link `sqlx` database triggers mechanically into the synchronous `eframe` render loop without blocking the 60FPS UI layout geometry!

## Phase 1: Context Mounting (`main.rs`)
Currently, `ChaissApp::new()` accepts no parameters. We will:
1. Initialize the `SqlitePool` driver struct (`chaiss_core::db::DbClient::new(...)`) strictly inside the `tokio::main` loop locally.
2. Bind the active client into a native `Arc<DbClient>` pointer mathematically locking it against thread lifetimes.
3. Pass the structure dynamically into `ChaissApp::new(cc, _db_client)` upon window launch.

## Phase 2: Asynchronous Struct Binding (`app.rs` via `flume`)
`egui` runs sequentially mathematically. We cannot block it waiting for SQLite to confirm a write action natively! 
1. We will install the highly-performant `flume` crate to bridge async/sync memory safely!
2. Define a native data structure `pub enum DbEvent { GameCreated { id: i64 }, MoveLogged }`.
3. Give `ChaissApp` access to `flume::unbounded()` tracking structures representing thread-safe data pipelines algebraically!
4. Pre-emptively sweep the `Receiver` end implicitly during `pub fn update(&mut self...)` parsing any new active Database confirmations via `try_recv()`.

## Phase 3: Trigger Geometries (`left_panel.rs` & `board.rs`)
**Game Creation:**
When "Confirm Starts Game" fires:
1. We invoke `tokio::spawn` spinning out a detached background thread locally! 
2. The cloned `Arc<DbClient>` searches for or builds both Black/White players, then sequentially writes the new `games` tuple.
3. It natively shoots a `DbEvent::GameCreated` down the transmitter!

**Logging Moves:**
Whenever you natively drop a piece down on the checkerboard (and Sandbox Mode is physically `false`!):
1. `tokio::spawn` fires.
2. It mathematically pulls the active `game_id`, algebraic text notation, and the explicit string layout bindings and pumps it directly into `DbClient::log_move()`.

## Phase 4: Verification
Launch `cargo run --bin desktop`. Verify creating a game spins up the active `sqlite3` tracker seamlessly!
