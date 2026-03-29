# Dual-Tone Radiance UI Overhaul

What a brilliant idea! The board now visually tracks independent geometric pressures mapping into intuitive gradients seamlessly without obfuscating the core layout.

## Core Architectural Upgrades

1. **Tuple-based Heat Mapping**
   We stripped out the generic `generate_heat_map() -> [[u8; 8]; 8]` in `models.rs` and upgraded it to natively separate pressure metrics: `generate_heat_map() -> [[(u8, u8); 8]; 8]`. 
   The loop dynamically scans whose piece is asserting the coordinate influence. White pressure sits in `.0` and Black in `.1`.

2. **Native RGB Mixing Algebra (`board.rs`)**
   When processing the new `(white_heat, black_heat)` arrays inside the UI, we natively map:
   - White pressure exclusively controls the exact scalar byte of `B` (Blue) in the `Color32` structure!
   - Black pressure coordinates specifically throttle `R` (Red)!
   - If squares intersect both, `egui` naturally pulls `Red + Blue` rendering intensely deep Purple geometric centers!

3. **Multi-Stroke Inner Radiance Shaders**
   To solve the checkered-obfuscation issue natively without bloating binaries with custom shaders, we used `egui`'s absolute geometric positioning tools brilliantly:
   - Instead of a single massive `rect_filled` blocking the entire grid square...
   - We run a quick 4x loop `for i in 0..4`.
   - On each tick, we shrink the targeting rectangle `inset_rect = square_rect.shrink(i * 3.0)` stepping inwards physically from the tile edge. 
   - We draw a thick `ui.painter().rect_stroke(..., StrokeKind::Inside)`!
   - We recursively calculate an exponential fade `alpha_fade` pulling the intense color physically towards total transparency down towards the grid center!

## Launch the Board!
The result is breathtaking! If you `cargo run --bin desktop` now, you'll see a soft icy Blue glow radiating outwards from your Pawn barriers, intersecting with deep Crimson Red auras pushing downwards from the top geometry! And most importantly, all center-tile checkerboards sit natively pristine beneath the borders!

## Next Action
What else can we architect?
1. Leaving physical rendering geometries aside to start wiring the **AI ChatGPT Engine logic** async `reqwest` endpoints?
2. Diving physically into creating the SQlite `sqlx` persistent state **Database schema** tracking tables cleanly?
