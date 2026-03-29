# Implementation Plan: Dual-Tone Heatmap Radiance

The current UI obscures the geometrical board layout when intense heat maps are generated. Furthermore, all pressure is combined into a single visual scalar, obfuscating which player actually controls the square!

## Phase 1: Engine Logic (`models.rs`)
1. **Refactor Heat Map Return Type:**
   Change `pub fn generate_heat_map(&self) -> [[u8; 8]; 8]` to `pub fn generate_heat_map(&self) -> [[(u8, u8); 8]; 8]`.
   The tuple `(White_Heat, Black_Heat)` explicitly splits the mathematical threat scalars.
2. **Scanner Updates:** 
   Iterate `0..64`. If `piece.color == White`, increment the `White_Heat` tuple slot on the attacked squares. If `Black`, increment the `Black_Heat` slot.

## Phase 2: GUI Rendering & Radiance Simulation (`board.rs`)
1. **Extracting Colors:**
   When rendering `square_rect` inside `desktop/src/ui/board.rs`, we fetch `let (white_heat, black_heat) = heat_map[rank][file]`.
   - Calculate Blue intensity from `white_heat`.
   - Calculate Red intensity from `black_heat`.
   - This natively forms Purple (`R + B`) when mathematical pressure overlaps symmetrically!
2. **Radiance Effect:**
   Instead of `ui.painter().rect_filled(square_rect, ...)` which obfuscates the checkered base, we will draw inward-shrinking nested borders using `egui::Rect::shrink(...)`. 
   By drawing 4-5 semi-transparent strokes stepping inwards from the edges, we beautifully simulate a glowing inner-radiance border organically inside `egui` without needing heavy external shader permutations!

## Phase 3: Verification
Verify through compiling `cargo run --bin desktop` that:
1. The heat naturally hugs the edges of the checkered tiles.
2. The grid pattern is 100% visible in the center of the tiles.
3. Central tensions (e.g., `e4` / `d5` early game overlaps) beautifully burn purple!
