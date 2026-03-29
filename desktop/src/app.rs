use eframe::egui;
use crate::ui;
use chaiss_core::engine::GameState;

#[derive(Clone)]
pub struct ChaissApp {
    prompt_buffer: String,
    game_state: GameState,
}

impl Default for ChaissApp {
    fn default() -> Self {
        Self {
            prompt_buffer: String::new(),
            game_state: GameState::new(),
        }
    }
}

impl ChaissApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ChaissApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render order: Outer Side Panels first, then Central Panel consumes remainder.
        ui::left_panel::draw(ctx);
        ui::right_panel::draw(ctx, &mut self.prompt_buffer);
        ui::board::draw(ctx, &self.game_state);
    }

    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Stub to satisfy eframe 0.34 App trait signature
    }
}
