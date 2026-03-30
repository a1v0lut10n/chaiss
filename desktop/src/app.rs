use eframe::egui;
use crate::ui;
use chaiss_core::engine::GameState;
use chaiss_core::db::{DbClient, GameRecord};
use std::sync::Arc;

pub enum DbEvent {
    GameCreated { game_id: i64 },
    SessionsLoaded { sessions: Vec<GameRecord> },
    GameResumed { history: Vec<String>, game_id: i64 },
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
    pub active_sessions: Vec<GameRecord>,
    
    // UI Presentation
    pub flip_board: bool,
    
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
            active_sessions: Vec::new(),
            history_stack: Vec::new(),
            view_cursor: 0,
            sandbox_enabled: false,
            is_exploration_mode: false,
            flip_board: false,
        }
    }
}

impl ChaissApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, db_client: Arc<DbClient>, initial_sessions: Vec<GameRecord>) -> Self {
        let (tx, rx) = flume::unbounded();
        let mut app = Self::default();
        app.db_client = Some(db_client);
        app.db_tx = Some(tx);
        app.db_rx = Some(rx);
        app.active_sessions = initial_sessions;
        app
    }
}

impl eframe::App for ChaissApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Explicitly receive background database operations mathematically across the barrier!
        if let Some(rx) = &self.db_rx {
            while let Ok(event) = rx.try_recv() {
                match event {
                    DbEvent::GameCreated { game_id } => {
                        self.active_game_id = Some(game_id);
                        println!("SQL Resolution Acquired Natively! Bound Game ID: {}", game_id);
                        
                        // Automatically fire a Flume DB fetch algebraically to dynamically inject the updated roster UI!
                        if let (Some(db), Some(tx)) = (self.db_client.clone(), self.db_tx.clone()) {
                            tokio::spawn(async move {
                                if let Ok(sessions) = db.get_active_games().await {
                                    let _ = tx.send_async(DbEvent::SessionsLoaded { sessions }).await;
                                }
                            });
                        }
                    }
                    DbEvent::SessionsLoaded { sessions } => {
                        self.active_sessions = sessions;
                        println!("Active SQLite Sessions completely refreshed & injected Egui natively!");
                    }
                    DbEvent::GameResumed { history, game_id } => {
                        self.active_game_id = Some(game_id);
                        self.history_stack = history;
                        
                        // Mutate active board layout to exactly match the final chronological move algebraically!
                        if let Some(final_fen) = self.history_stack.last() {
                            self.game_state = GameState::from_fen(final_fen).expect("Malformed Final Historical Frame Array!");
                        } else {
                            self.game_state = GameState::new();
                        }
                        
                        // Hard resynchronize constraints cleanly
                        self.live_db_ply = self.history_stack.len();
                        self.view_cursor = self.history_stack.len().saturating_sub(1);
                        self.sandbox_enabled = false;
                        self.is_exploration_mode = false;
                        
                        println!("Game {} dynamically cleanly resurrected dynamically from Cold Storage!", game_id);
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
