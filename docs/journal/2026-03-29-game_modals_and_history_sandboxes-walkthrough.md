# UI Scaffolding: Game Modals & History Sandboxes

The architecture for handling persistent matches and hypothetical sandbox explorations has been fully geometrically integrated into the `egui` native loop!

## 1. Relational Naming Upgrades
As planned, I completely erased and re-generated your local `chaiss.db` instance, modifying the `games` table to include `name TEXT NOT NULL DEFAULT 'Untitled Game'`. I then cleanly mapped this parameter deep into `core/src/db.rs` so the SQLite macro inherently binds Strings straight into the backend query natively!

## 2. Dynamic Memory Tracking
Inside `desktop/src/app.rs`, `ChaissApp` now holds native buffering context tracking:
- `show_new_game_modal: bool`
- `active_game_id`, `new_game_name`, `white_player_name`, `black_player_name`
- `<Vec> history_stack` (Our complete list of FENs for the session!)
- `view_cursor: usize` (Which move we are physically looking at).

Because of this, right at the top of the update loop, the app natively flags:
`self.is_exploration_mode = self.view_cursor < self.history_stack.len() - 1`.

## 3. The Front-End Elements
Check out the active `cargo run` window!
- Click **"Create New Game"** on the left panel! A gorgeous, native floating modal will instantly spawn dynamically prompting you for explicit text-inputs for White/Black and the Game Title. Clicking `Cancel` cleanly hides it via boolean toggling.
- Check the very bottom of the **Board Geometry**! You will now proudly witness the `Sandbox Navigation` block mechanically bound there. 

### The Exploration Control Block
Beneath the grid sits: **`[<< Start]` `[< Prev Move]` `[Next Move >]` `[Live >>]`**
Above it, a huge text-tag warns you: 
> `EXPLORATION MODE ACTIVE (Not saving to Database)` (Rendered in Warning Orange!)
or
> `LIVE DB TRACKING` (Rendered in Safe Green!).

Because I wired these bounds explicitly natively into `board.rs`, when we physically sync these triggers to the async Database handlers and `Tokio` runtime, you will be able to flawlessly rewind FEN physical histories mathematically!

## Next Objective
With the physical `egui` layouts mathematically aligned to our Database tracking schemas, what is the best order of operations?
1. Start physically wiring the UI triggers to actual async `DbClient` execution queries (e.g. actually generating the database rows when you click "Confirm Starts Game")?
2. Swapping focus over to the `llm.rs` engine and giving the `tokio` layer its first test communicating `FEN` strings over the network to a language model AI?
