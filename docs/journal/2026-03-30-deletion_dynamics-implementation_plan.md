# Deletion Dynamics

Currently, our SQLite layer structurally captures every session created. To physically fulfill the UI feature map, we need to bridge a `DELETE` pipeline tracking back from the Session Roster explicitly into the persistent mathematical matrices.

## User Review Required
Does a simple, right-aligned `🗑` (Trash Can) icon nested directly alongside the individual session Title buttons structurally fulfill the objective natively? 

## Proposed Changes

### 1. Database Primitives (`core/src/db.rs`)
I will physically build an asynchronous `delete_game` algebraic resolver natively handling SQLite data execution. It safely expunges both the structural `moves` array and the parent `games` layout strictly via `DELETE FROM`.

### 2. Flume Event Piping (`desktop/src/app.rs`)
I will introduce `DbEvent::GameDeleted { game_id: i64 }` into the asynchronous engine structure natively. 
When the Async layer successfully executes the SQLite deletion physically, Egui intercepts the event frame and logically fires `get_active_games()` dynamically mathematically repopulating the SidePanel completely devoid of the purged match!
**Bonus Constraint**: If the User deletes the Game they are *currently actively playing*, the application dynamically sets `active_game_id = None` and resets the Board mathematically to a brand new slate natively! 

### 3. Structural Roster Layout (`desktop/src/ui/left_panel.rs`)
I will wrap the existing Session parsing logic natively inside an `ui.horizontal` layout. The existing dynamic `RichText` Name button will span 80% natively, while physically preserving a neat corner right-aligned button `egui::Button::new("🗑")`!

## Verification Plan
1. Compile structural engine bounds natively and trigger the Layout updates.
2. Initialize `cargo run --bin desktop`.
3. Create a dummy test game natively.
4. Click the `🗑` directly next to the active Session structurally explicitly verifying it physically strips cleanly off the Egui context array without triggering threading blocks locally!
