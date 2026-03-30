use eframe::egui;
use crate::ui;
use chaiss_core::engine::GameState;
use chaiss_core::db::DbClient;
use std::sync::Arc;

pub enum DbEvent {
    GameCreated { game_id: i64 },
}

#[derive(Clone)]
pub struct ChaissApp {
    pub prompt_buffer: String,
    pub game_state: GameState,
    pub selected_square: Option<usize>,
    
    // Asynchronous Database Flume Bridges
    pub db_client: Option<Arc<DbClient>>,
    pub db_tx: Option<flume::Sender<DbEvent>>,
    pub db_rx: Option<flume::Receiver<DbEvent>>,
    
    // UI Modals
    pub show_new_game_modal: bool,
    pub new_game_name: String,
    pub white_player_name: String,
    pub black_player_name: String,
    
    // Database Tracking
    pub active_game_id: Option<i64>,
    pub live_db_ply: usize, // Tracks the absolute length of mathematically committed DB moves
    
    // History & Exploration Sandbox
    pub history_stack: Vec<String>,
    pub view_cursor: usize,
    pub sandbox_enabled: bool,
    pub is_exploration_mode: bool,
}

impl Default for ChaissApp {
    fn default() -> Self {
        Self {
            prompt_buffer: String::new(),
            game_state: GameState::new(),
            selected_square: None,
            db_client: None,
            db_tx: None,
            db_rx: None,
            show_new_game_modal: false,
            new_game_name: "My First Game".to_string(),
            white_player_name: "Human Player".to_string(),
            black_player_name: "Chaiss GPT".to_string(),
            active_game_id: None,
            live_db_ply: 0,
            history_stack: Vec::new(),
            view_cursor: 0,
            sandbox_enabled: false,
            is_exploration_mode: false,
        }
    }
}

impl ChaissApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, db_client: Arc<DbClient>) -> Self {
        let (tx, rx) = flume::unbounded();
        let mut app = Self::default();
        app.db_client = Some(db_client);
        app.db_tx = Some(tx);
        app.db_rx = Some(rx);
        app
    }
}

impl eframe::App for ChaissApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Explicitly receive background database operations mathematically across the barrier!
        if let Some(rx) = &self.db_rx {
            if let Ok(event) = rx.try_recv() {
                match event {
                    DbEvent::GameCreated { game_id } => {
                        self.active_game_id = Some(game_id);
                        println!("SQL Resolution Acquired Natively! Bound Game ID: {}", game_id);
                    }
                }
            }
        }
        
        // Evaluate dynamic exploration mode natively before drawing layout!
        // You are in exploration if the user manually ticked Sandbox, OR if you scrolled back BEFORE the absolute live DB play vector!
        self.is_exploration_mode = self.sandbox_enabled || (self.history_stack.len() > 0 && self.view_cursor < self.live_db_ply.saturating_sub(1));

        ui::left_panel::draw(ctx, self);
        ui::right_panel::draw(ctx, &mut self.prompt_buffer);
        ui::board::draw(ctx, self);
    }

    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Stub to satisfy eframe 0.34 App trait signature
    }
}
