# SVG Vector Rendering Implementation Plan

Replacing our hardcoded Unicode characters with true vector SVGs achieves two massive goals:
1. **Pristine Scalability**: Pieces will natively snap sharply at ANY display scale, whereas our current `text()` function depends on logical font hinting heuristics.
2. **Visual Modifiers**: By using standard transparent SVGs, we can natively tint them dynamically in the future to map "Threatened" or "Favored" states directly over their sprites!

## User Review Required
Does the dependency tree injection look clean natively? We will need to pull down 12 standard FIDE visual SVGs (from Wikipedia/Wikimedia) mapping to standard `Pawn`, `Knight`, `Rook`, `King`, `Queen`, `Bishop` arrays!

## Proposed Changes

### 1. Cargo Pipeline Evolution (`desktop/Cargo.toml`)
Yes, Egui fundamentally natively parses generic Scalable Vector Graphics by dynamically passing the byte vectors mathematically into its internal raster pipeline! We inject:
#### [MODIFY] [Cargo.toml](file:///home/hansvw/Projects/Aivolution/chaiss/desktop/Cargo.toml)
```toml
egui_extras = { version = "0.34.1", features = ["svg", "image"] }
```

### 2. Startup Orchestration (`desktop/src/main.rs`)
To natively decode SVG XML physically onto Egui's GPU buffers mathematically, we'll actively bind the loaders during startup logic globally!
#### [MODIFY] [main.rs](file:///home/hansvw/Projects/Aivolution/chaiss/desktop/src/main.rs)
```rust
Box::new(move |cc| {
    egui_extras::install_image_loaders(&cc.egui_ctx);
    Ok(Box::new(ChaissApp::new(cc, db_client, initial_sessions)))
})
```

### 3. SVG Fetch & Asset Bundling
I will automatically fetch the 12 classic "FIDE standard" piece vectors digitally (using `curl`) and store them sequentially under `<root>/assets/pieces/` mathematically:
- `assets/pieces/bB.svg` (Black Bishop)
- `assets/pieces/wK.svg` (White King)
- ...etc.

### 4. Board Geometry Swapping (`desktop/src/ui/board.rs`)
Inside `board.rs`, we fundamentally replace the `ui.painter().text()` Unicode render physically:
#### [MODIFY] [board.rs](file:///home/hansvw/Projects/Aivolution/chaiss/desktop/src/ui/board.rs)
```rust
let piece_uri = match piece.color {
    Color::White => format!("file://assets/pieces/w{}.svg", piece.piece_type.to_char().to_uppercase()),
    Color::Black => format!("file://assets/pieces/b{}.svg", piece.piece_type.to_char().to_uppercase()),
};

// Directly mount SVGs onto our constrained tile box mathematically natively!
let image = egui::Image::new(piece_uri).fit_to_exact_size(egui::vec2(square_size * 0.9, square_size * 0.9));
ui.add(image); // Or paint directly if controlling layout anchors natively!
```
*(Wait: Because we act inside absolute grids natively, we will either shift to `Widget` anchors or parse it onto the `Painter` mathematically via `egui::Image` mapped via texture handles).*

## Verification Plan
1. Download all required `.svg` FIDE standard vector math components to local storage.
2. Inject the dynamic Egui loader pipeline.
3. Replace the `Painter::text` natively.
4. Hot-reload the Window locally and mathematically confirm SVGs flawlessly rasterize at arbitrary zoom scales!
