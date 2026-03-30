# Implementation Plan: Active Session Rendering

To successfully breathe life into the "Active Sessions" roster in Egui natively, we need to bridge SQLite dynamic `SELECT` queries across into Egui's local structures!

## Phase 1: Database Struct Definition (`core/src/db.rs`)
1. Create `pub struct GameRecord { pub id: i64, pub name: String, pub white_player: String, pub black_player: String, pub status: String }`.
2. Construct `pub async fn get_active_games(&self) -> Result<Vec<GameRecord>, sqlx::Error>`.
3. This function uses SQL `JOIN` mapping `games` to `players` perfectly natively, sorted by `updated_at DESC`.

## Phase 2: Resume History Mapping (`core/src/db.rs`)
To allow the UI to fundamentally restore the exact state of a game (including its full Sandbox history vector):
1. Construct `pub async fn load_game_history(&self, game_id: i64) -> Result<Vec<String>, sqlx::Error>`.
2. This fetches every `fen_snapshot` sequentially mapped in the `moves` table natively!
3. The UI will forcibly insert the Base FIDE layout string natively as the zero-index element mathematically.

## Phase 3: Egui Channel Structuring (`desktop/src/app.rs`)
1. Add `DbEvent::SessionsLoaded { sessions: Vec<GameRecord> }` to our `flume` bridge natively.
2. Add `DbEvent::GameResumed { history: Vec<String>, game_id: i64 }` mechanically.
3. Inject `pub active_sessions: Vec<GameRecord>` onto `ChaissApp`.

## Phase 4: Trigger Bindings
1. **Initial Load**: During `eframe` window bootup, we push a `tokio::spawn(get_active_games())` immediately over the wire dynamically!
2. **Left Panel Population**: We structurally wipe the `for i in 0..5` placeholders natively and iterate over `app.active_sessions`.
3. **Session Re-Sync**: Whenever a user successfully creates a *new* game, the identical DB thread fires *another* `get_active_games()` structurally, throwing an identical event locally updating the array!
