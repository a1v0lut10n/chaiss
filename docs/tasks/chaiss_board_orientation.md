# Dynamic Board Orientation & Turn Tracking

During physical gameplay or match review, the user should be able to invert the visual Cartesian coordinate system so that Black starts at the bottom of the screen. We also need constant Egui state tracking whose turn it algebraically is.

## 1. State Tracking (`desktop/src/app.rs`)
We add a simple boolean:
```rust
pub flip_board: bool
```
This is easily toggled via a button. 

## 2. Geometry Manipulation (`desktop/src/ui/board.rs`)
The Egui painter generates grids from Top-Left to Bottom-Right (`row 0..8` and `col 0..8`).
If we change nothing about our drawing loops physically, we can dynamically invert the `index` we pull from our GameState arrays based on the `flip_board` boolean map natively:

### Grid Re-Mapping
```rust
let logical_row = if app.flip_board { 7 - row } else { row };
let logical_col = if app.flip_board { 7 - col } else { col };
let index = logical_row * 8 + logical_col;
```
By doing this, the UI logic transparently pulls Black's pieces (Ranks 8 & 7) to the bottom rows visually without altering the underlying FIDE math strings!

### Coordinate File & Rank Alterations
We must perfectly invert the A-H & 1-8 logic natively too:
```rust
let logical_col = if app.flip_board { 7 - col } else { col };
let file_char = (b'a' + logical_col as u8) as char;

let logical_row = if app.flip_board { 7 - row } else { row };
let rank_char = (b'8' - logical_row as u8) as char;
```

## 3. Active Turn Indication (`desktop/src/ui/board.rs`)
We will allocate explicit UI geometry (perhaps right below the top-header or alongside the Navigation UI) that reads:
**`Current Turn: ⬜ White`** or **`⬛ Black`**.
We can read this dynamically from `app.game_state.active_color`.
