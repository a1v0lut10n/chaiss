# Dynamic Perspective Architecture

In order to physically permit users to assume the "Black" perspective seamlessly, we will programmatically invert the visual matrix bindings without altering the core mathematical indexing of `core/src/GameState` structurally natively!

## User Review Required
Does the visual UI layout for the "Flip Board" button placement work for you structurally? I propose placing it neatly next to the Navigation / Sandbox controls algebraically.

## Proposed Changes

### 1. State Expansion (`desktop/src/app.rs`)
We introduce a `flip_board: bool` primitive onto `ChaissApp`. It defaults to `false` (White at the bottom natively).

### 2. Physical Axis Inversion (`desktop/src/ui/board.rs`)
Egui natively renders `rect` geometry from top-to-bottom physically. We will keep the visual loop constraints structurally constant, but mutate the Mathematical pointer exactly:

#### The Grid Fetcher
```rust
// Instead of index = row * 8 + col;
let logical_row = if app.flip_board { 7 - row } else { row };
let logical_col = if app.flip_board { 7 - col } else { col };
let index = logical_row * 8 + logical_col;
```
This forces the renderer to structurally query index `63` (h1) when physically rendering at loop `(0, 0)` if `flip_board` is true! It inherently handles Mouse hitbox bounds, Drag logic, and rendering intrinsically because `index` intrinsically ties back to the legal mathematical arrays organically!

#### The Axis Annotations
We apply identical mirroring physically to both the `.text(...)` coordinate loops:
```rust
let logical_col = if app.flip_board { 7 - col } else { col };
let file_char = (b'a' + logical_col as u8) as char;
```

### 3. Turn Tracking Overlay (`desktop/src/ui/board.rs`)
We will extract the mathematically derived `app.game_state.active_color` enum.
Near the Checkmate and Navigation panels structurally, we'll draw a beautifully styled `egui::RichText` dynamically indicating:
**`Active Turn: ⬜ White to Move`** (or Black natively).

## Verification Plan
1. Construct the boolean structures natively.
2. Mathematically map the inversion tuples structurally into `board.rs`.
3. Build the Application locally and visually toggle the `Flip Board` button.
4. Verify standard FIDE annotations (`a-h`, `1-8`) mirror exactly backwards gracefully around the grid bounds.
5. Verify clicking and dragging algebraic pieces successfully tracks geometries locally without coordinate corruption natively.
