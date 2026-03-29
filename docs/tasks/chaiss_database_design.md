# Implementation Plan: Persistent Schema Design (SQLite)

This phase establishes the localized data integrity schema so Chess board states survive application restarts. Building robust relational tables is also critical for LLM context aggregation (feeding historical moves directly to ChatGPT!).

## Phase 1: Environment & Tooling Scaffold
- Since we are utilizing `sqlx` natively requiring compile-time query verification, we must formally structure a `.env` file containing `DATABASE_URL=sqlite://chaiss.db`.
- We will construct the `migrations/` folder manually tracking the creation of `players`, `games`, and `moves`.

## Phase 2: Relational Schema Mapping
We will define three heavily normalized native SQLite tables:

### 1. `players`
Tracking human and AI participants.
- `id` (INTEGER PRIMARY KEY)
- `name` (TEXT UNIQUE)
- `created_at` (TEXT)

### 2. `games`
The active umbrella for a chess bout.
- `id` (INTEGER PRIMARY KEY)
- `white_player_id` (INTEGER REFERENCES `players`)
- `black_player_id` (INTEGER REFERENCES `players`)
- `current_fen` (TEXT) -> Perfect state memory snapshot!
- `status` (TEXT) -> E.g., `ongoing`, `checkmate_white`, `stalemate`
- `created_at` (TEXT), `updated_at` (TEXT)

### 3. `moves` (The History Stack)
A highly granular ledger of every single action taken on the chess board natively.
- `id` (INTEGER PRIMARY KEY)
- `game_id` (INTEGER REFERENCES `games`)
- `move_number` (INTEGER)
- `fen_snapshot` (TEXT) -> **CRITICAL**: Storing the exact mathematical string layout *after* this move executes allows us to trivially query the row backward and inject it into `GameState::from_fen()` for **Undoing moves** perfectly!
- `notation` (TEXT) -> The exact vector logged (e.g., `e2e4` or FIDE algebraic).
- `created_at` (TEXT)

## Phase 3: Driver Integration (`db.rs`)
Inside `chaiss_core/src/db.rs`, we will initialize the `sqlx::sqlite::SqlitePool` structurally.
We will build foundational CRUD queries verified strictly at compile time:
- `pub async fn create_player(...)`
- `pub async fn create_game(...)`
- `pub async fn log_move(...)`
- `pub async fn load_game_by_id(...) -> GameState`
- `pub async fn undo_last_move(game_id: i64) -> GameState` -> Deletes the top row from `moves` and fetches the `fen_snapshot` of the previous row directly!

## Phase 4: Verification
- Instantiating an SQL script and generating the exact schema locally `chaiss.db`.
- Running an active `cargo check` verifying that `sqlx::query!` macros securely bind natively over the schema tables without type faults!
