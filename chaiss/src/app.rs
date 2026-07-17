use crate::ui;
use chaiss_core::db::{DbClient, GameRecord};
use chaiss_core::engine::GameState;
use eframe::egui;
use std::sync::Arc;

pub enum DbEvent {
    GameCreated {
        game_id: i64,
    },
    GameDeleted {
        game_id: i64,
    },
    SessionsLoaded {
        sessions: Vec<GameRecord>,
    },
    GameResumed {
        history: Vec<String>,
        algebraic: Vec<String>,
        chat: Vec<(String, String)>,
        game_id: i64,
    },
}

#[derive(PartialEq, Clone, Copy)]
pub enum FocusMatrix {
    None,
    FirstOrder,
    Predictive,
}

pub enum LlmEvent {
    InferenceRequested(chaiss_core::llm::LlmPromptPayload),
    TokenStreamed(String),
    SystemMessage(String),
    InferenceFinished,
}

pub struct ChaissApp {
    pub prompt_buffer: String,
    pub game_state: GameState,
    pub selected_square: Option<usize>,

    // Asynchronous Database Flume Bridges
    pub db_client: Option<Arc<DbClient>>,
    pub db_tx: Option<flume::Sender<DbEvent>>,
    pub db_rx: Option<flume::Receiver<DbEvent>>,

    // Asynchronous LLM Flume Bridges
    pub llm_tx: Option<flume::Sender<LlmEvent>>,
    pub llm_rx: Option<flume::Receiver<LlmEvent>>,
    pub chat_history: Vec<(String, String)>,
    pub live_llm_response: String,
    pub silence_llm_analysis: bool,
    pub markdown_cache: egui_commonmark::CommonMarkCache,

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
    pub algebraic_history: Vec<String>,
    pub view_cursor: usize,
    pub sandbox_enabled: bool,
    pub is_exploration_mode: bool,
    pub is_llm_thinking: bool,

    // Matrix Visualization Architecture
    pub focus_matrix: FocusMatrix,
    pub ai_predictive_arrows: Vec<(usize, usize)>,

    // Retry and Auto-Healing Mechanics
    pub retry_count: usize,
    pub active_payload: Option<chaiss_core::llm::LlmPromptPayload>,
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
            llm_tx: None,
            llm_rx: None,
            chat_history: Vec::new(),
            live_llm_response: String::new(),
            silence_llm_analysis: false,
            markdown_cache: egui_commonmark::CommonMarkCache::default(),
            show_new_game_modal: false,
            new_game_name: "My First Game".to_string(),
            white_player_name: "Human Player".to_string(),
            black_player_name: "Chaiss GPT".to_string(),
            active_game_id: None,
            live_db_ply: 0,
            active_sessions: Vec::new(),
            history_stack: Vec::new(),
            algebraic_history: Vec::new(),
            view_cursor: 0,
            sandbox_enabled: false,
            is_exploration_mode: false,
            is_llm_thinking: false,
            flip_board: false,
            focus_matrix: FocusMatrix::FirstOrder,
            ai_predictive_arrows: Vec::new(),
            retry_count: 0,
            active_payload: None,
        }
    }
}

impl ChaissApp {
    pub fn sanitize_markdown(input: &str) -> String {
        let mut sanitized = strip_math_spans(input.trim_start());
        let backtick_count = sanitized.matches("```").count();
        if !backtick_count.is_multiple_of(2) {
            if !sanitized.ends_with('\n') {
                sanitized.push('\n');
            }
            sanitized.push_str("```\n");
        }
        sanitized
    }

    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        db_client: Arc<DbClient>,
        initial_sessions: Vec<GameRecord>,
    ) -> Self {
        let (tx, rx) = flume::unbounded();
        let (llm_tx, llm_rx) = flume::unbounded();
        let mut app = Self {
            db_client: Some(db_client),
            ..Default::default()
        };
        // This crucially synthetically trips the `active_game_id.is_none()` resolver logic natively on frame 1!
        let _ = tx.send(DbEvent::SessionsLoaded {
            sessions: initial_sessions.clone(),
        });

        app.db_tx = Some(tx);
        app.db_rx = Some(rx);
        app.llm_tx = Some(llm_tx);
        app.llm_rx = Some(llm_rx);
        app.active_sessions = initial_sessions;
        app
    }
}

impl eframe::App for ChaissApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 1. Explicitly receive background database operations mathematically across the barrier!
        if let Some(rx) = &self.db_rx {
            while let Ok(event) = rx.try_recv() {
                match event {
                    DbEvent::GameCreated { game_id } => {
                        self.active_game_id = Some(game_id);
                        println!("Created new match with ID: {}", game_id);

                        // Break mathematical chat matrices natively initializing pure session bindings!
                        self.game_state = GameState::new();
                        self.history_stack.clear();
                        self.algebraic_history.clear();
                        self.live_db_ply = 0;
                        self.view_cursor = 0;
                        self.selected_square = None;
                        self.chat_history.clear();
                        self.live_llm_response.clear();
                        self.prompt_buffer.clear();
                        self.ai_predictive_arrows.clear();

                        // Automatically fire a Flume DB fetch algebraically to dynamically inject the updated roster UI!
                        if let (Some(db), Some(tx)) = (self.db_client.clone(), self.db_tx.clone()) {
                            tokio::spawn(async move {
                                if let Ok(sessions) = db.get_active_games().await {
                                    let _ =
                                        tx.send_async(DbEvent::SessionsLoaded { sessions }).await;
                                }
                            });
                        }
                    }
                    DbEvent::GameDeleted { game_id } => {
                        println!("Deleted game with ID: {}", game_id);

                        // Break mathematical ties completely if we delete the Active viewing matrix!
                        if self.active_game_id == Some(game_id) {
                            self.active_game_id = None;
                            self.game_state = GameState::new();
                            self.history_stack.clear();
                            self.algebraic_history.clear();
                            self.chat_history.clear();
                            self.live_llm_response.clear();
                            self.prompt_buffer.clear();
                            self.ai_predictive_arrows.clear();
                            self.selected_square = None;
                            self.live_db_ply = 0;
                            self.view_cursor = 0;
                            self.is_exploration_mode = false;
                        }

                        // Mathematically refresh structural Egui Sessions arrays organically!
                        if let (Some(db), Some(tx)) = (self.db_client.clone(), self.db_tx.clone()) {
                            tokio::spawn(async move {
                                if let Ok(sessions) = db.get_active_games().await {
                                    let _ =
                                        tx.send_async(DbEvent::SessionsLoaded { sessions }).await;
                                }
                            });
                        }
                    }
                    DbEvent::SessionsLoaded { sessions } => {
                        self.active_sessions = sessions.clone();
                        println!("Loaded active sessions from database.");

                        // On cold boot, automatically deserialize the most recent mathematical Match explicitly!
                        if self.active_game_id.is_none() && !self.active_sessions.is_empty() {
                            let latest_id = self.active_sessions[0].id;
                            if let (Some(db), Some(tx)) =
                                (self.db_client.clone(), self.db_tx.clone())
                            {
                                tokio::spawn(async move {
                                    if let Ok((root_fen, mut history, mut algebraic)) =
                                        db.load_game_history(latest_id).await
                                    {
                                        history.insert(0, root_fen);
                                        algebraic.insert(0, "START".to_string());
                                        let chat = db
                                            .load_chat_history(latest_id)
                                            .await
                                            .unwrap_or_default();
                                        let _ = tx
                                            .send_async(DbEvent::GameResumed {
                                                history,
                                                algebraic,
                                                chat,
                                                game_id: latest_id,
                                            })
                                            .await;
                                    }
                                });
                            }
                        }
                    }
                    DbEvent::GameResumed {
                        history,
                        algebraic,
                        chat,
                        game_id,
                    } => {
                        self.active_game_id = Some(game_id);
                        self.history_stack = history;
                        self.algebraic_history = algebraic;

                        self.chat_history = chat;
                        self.live_llm_response.clear();
                        self.prompt_buffer.clear();
                        self.ai_predictive_arrows.clear();
                        self.selected_square = None;

                        if let Some(final_fen) = self.history_stack.last() {
                            self.game_state = GameState::from_fen(final_fen)
                                .expect("Malformed Final Historical Frame Array!");
                        } else {
                            self.game_state = GameState::new();
                        }

                        // Restore Purely Algebraic Resignation / Game-Over Vectors!
                        if let Some(last_move) = self.algebraic_history.last() {
                            if last_move == "1-0" {
                                self.game_state.manual_terminal_status =
                                    Some(chaiss_core::engine::GameEndStatus::Resignation(
                                        chaiss_core::engine::Color::White,
                                    ));
                            } else if last_move == "0-1" {
                                self.game_state.manual_terminal_status =
                                    Some(chaiss_core::engine::GameEndStatus::Resignation(
                                        chaiss_core::engine::Color::Black,
                                    ));
                            } else if last_move == "1/2-1/2" {
                                self.game_state.manual_terminal_status =
                                    Some(chaiss_core::engine::GameEndStatus::Stalemate);
                            }
                        }

                        // Hard resynchronize constraints cleanly
                        self.live_db_ply = self.history_stack.len();
                        self.view_cursor = self.history_stack.len().saturating_sub(1);
                        self.sandbox_enabled = false;
                        self.is_exploration_mode = false;

                        // Scan for any previously recorded predictive geometry inherently bridging across session bounds!
                        for (role, msg) in self.chat_history.iter().rev() {
                            if role == "Agent" {
                                if let Some(matrix_idx) = msg.find("### PREDICTIVE MATRIX:") {
                                    let substring =
                                        &msg[matrix_idx + "### PREDICTIVE MATRIX:".len()..];
                                    let sequence: Vec<&str> =
                                        substring.split(',').map(|s| s.trim()).collect();

                                    let mut sim_state = self.game_state.clone();
                                    for ply in sequence {
                                        let clean_ply =
                                            ply.replace(|c: char| !c.is_alphanumeric(), "");
                                        if let Ok((from, to, promo)) =
                                            chaiss_core::engine::notation::parse_algebraic_move(
                                                &sim_state, &clean_ply,
                                            )
                                        {
                                            self.ai_predictive_arrows.push((from, to));
                                            sim_state.apply_move(from, to, promo);
                                        } else {
                                            break;
                                        }
                                    }
                                }
                                break; // Only mathematically parse the definitive *latest* geometrical inference!
                            }
                        }

                        println!("Resumed game {} from database.", game_id);
                    }
                }
            }
        }

        // 2. Mathematically stream non-blocking Network LLM responses straight onto Egui Geometry safely!
        if let Some(rx) = &self.llm_rx {
            while let Ok(event) = rx.try_recv() {
                match event {
                    LlmEvent::InferenceRequested(payload) => {
                        if self.retry_count == 0 {
                            self.chat_history
                                .push(("User".to_string(), payload.prompt.clone()));

                            // Serialize user payload asynchronously into active match cleanly
                            if let (Some(db), Some(game_id)) =
                                (self.db_client.clone(), self.active_game_id)
                            {
                                let p_clone = payload.prompt.clone();
                                tokio::spawn(async move {
                                    let _ = db.log_chat_message(game_id, "User", &p_clone).await;
                                });
                            }
                        }

                        self.active_payload = Some(payload.clone());
                        self.live_llm_response = String::new();
                        self.is_llm_thinking = true;

                        // Break memory locks extracting standard pointers
                        let tx_clone = self.llm_tx.clone().unwrap();
                        let prompt_print = payload.prompt.clone();

                        tokio::spawn(async move {
                            // Forward tokens mathematically via a sub-channel internally!
                            let (stream_tx, stream_rx) = flume::unbounded::<String>();
                            let tx_clone_2 = tx_clone.clone();

                            // Spin isolated engine pipeline structurally fetching REST geometry offline
                            tokio::spawn(async move {
                                if let Err(e) = chaiss_core::llm::stream_llm_response(
                                    payload,
                                    stream_tx.clone(),
                                )
                                .await
                                {
                                    let _ = tx_clone_2
                                        .send_async(LlmEvent::SystemMessage(format!(
                                            "[System Error: {}]",
                                            e
                                        )))
                                        .await;
                                } else {
                                    let _ =
                                        tx_clone_2.send_async(LlmEvent::InferenceFinished).await;
                                }
                            });

                            // Re-bundle raw bytes explicitly streaming Egui mathematically!
                            while let Ok(token) = stream_rx.recv_async().await {
                                let _ = tx_clone.send_async(LlmEvent::TokenStreamed(token)).await;
                            }
                        });

                        println!("Dispatched LLM prompt: {}", prompt_print);
                    }
                    LlmEvent::TokenStreamed(token) => {
                        self.live_llm_response.push_str(&token);
                    }
                    LlmEvent::SystemMessage(msg) => {
                        self.chat_history.push(("System".to_string(), msg));
                        self.is_llm_thinking = false;
                        self.live_llm_response.clear();
                    }
                    LlmEvent::InferenceFinished => {
                        let has_matrix = self
                            .live_llm_response
                            .find("### PREDICTIVE MATRIX:")
                            .is_some();

                        if !has_matrix && self.retry_count < 3 {
                            println!("Incomplete response detected natively, auto-retrying... (Attempt {})", self.retry_count + 1);
                            self.retry_count += 1;
                            if let (Some(payload), Some(tx)) =
                                (self.active_payload.clone(), self.llm_tx.clone())
                            {
                                let _ = tx.send(LlmEvent::InferenceRequested(payload));
                            }
                            continue; // Skip the rest of the finishing logic
                        }

                        self.retry_count = 0;
                        self.active_payload = None;

                        let sanitized_response = Self::sanitize_markdown(&self.live_llm_response);

                        self.chat_history
                            .push(("Agent".to_string(), sanitized_response.clone()));
                        self.is_llm_thinking = false;

                        // Parse visual geometrical continuations structurally from the inference!
                        self.ai_predictive_arrows.clear();
                        if let Some(matrix_idx) = sanitized_response.find("### PREDICTIVE MATRIX:")
                        {
                            let substring =
                                &sanitized_response[matrix_idx + "### PREDICTIVE MATRIX:".len()..];
                            let sequence: Vec<&str> =
                                substring.split(',').map(|s| s.trim()).collect();

                            let mut sim_state = self.game_state.clone();
                            for ply in sequence {
                                // Strip punctuation mathematically just in case!
                                let clean_ply = ply.replace(|c: char| !c.is_alphanumeric(), "");
                                if let Ok((from, to, promo)) =
                                    chaiss_core::engine::notation::parse_algebraic_move(
                                        &sim_state, &clean_ply,
                                    )
                                {
                                    self.ai_predictive_arrows.push((from, to));
                                    sim_state.apply_move(from, to, promo);
                                } else {
                                    break; // Discard sequence securely tracking geometry bounds if parse structurally fails natively
                                }
                            }
                        }

                        // Serialize AI payload asynchronously tracking dynamic streams cleanly!
                        if let (Some(db), Some(game_id)) =
                            (self.db_client.clone(), self.active_game_id)
                        {
                            let r_clone = sanitized_response.clone();
                            tokio::spawn(async move {
                                let _ = db.log_chat_message(game_id, "Agent", &r_clone).await;
                            });
                        }

                        self.live_llm_response.clear();
                    }
                }
            }
        }

        // Evaluate dynamic exploration mode natively before drawing layout!
        // You are in exploration if the user manually ticked Sandbox, OR if you scrolled back BEFORE the absolute live DB play vector!
        self.is_exploration_mode = self.sandbox_enabled
            || (!self.history_stack.is_empty()
                && self.view_cursor < self.live_db_ply.saturating_sub(1));

        ui::left_panel::draw(ui, self);
        ui::right_panel::draw(ui, self);
        ui::board::draw(ui, self);
    }
}

/// Gemini wraps chess lines in TeX math (`$$\text{7. d4 \quad exd4}$$`) which
/// egui_commonmark renders verbatim; rewrite complete math spans as bold plain
/// text. Only complete delimiter pairs are rewritten — a partially streamed
/// span is left untouched and picked up once its closing delimiter arrives,
/// since the sanitizer re-runs over the full buffer every frame.
fn strip_math_spans(input: &str) -> String {
    // Segments between ``` fences alternate outside/inside code; only touch
    // the outside ones so dollar signs in code samples survive.
    input
        .split("```")
        .enumerate()
        .map(|(i, seg)| {
            if i % 2 == 0 {
                strip_math_in_text(seg)
            } else {
                seg.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("```")
}

fn strip_math_in_text(seg: &str) -> String {
    let mut out = String::with_capacity(seg.len());
    let mut rest = seg;
    while let Some(j) = rest.find('$') {
        // A backslash-escaped \$ is a literal dollar, not a delimiter.
        if rest[..j].ends_with('\\') {
            out.push_str(&rest[..j + 1]);
            rest = &rest[j + 1..];
            continue;
        }
        out.push_str(&rest[..j]);
        rest = &rest[j..];

        if let Some(body) = rest.strip_prefix("$$") {
            match body.find("$$") {
                Some(k) => {
                    let detexed = detex(&body[..k]);
                    if !detexed.is_empty() {
                        out.push_str("**");
                        out.push_str(&detexed);
                        out.push_str("**");
                    }
                    rest = &body[k + 2..];
                }
                None => {
                    // Unterminated display span: still streaming, keep verbatim.
                    out.push_str(rest);
                    return out;
                }
            }
        } else {
            let body = &rest[1..];
            match inline_math_end(body) {
                Some(k) => {
                    out.push_str("**");
                    out.push_str(&detex(&body[..k]));
                    out.push_str("**");
                    rest = &body[k + 1..];
                }
                None => {
                    out.push('$');
                    rest = body;
                }
            }
        }
    }
    out.push_str(rest);
    out
}

/// Find the closing `$` of an inline span, applying Pandoc-style validity
/// rules so ordinary prices ("$5 and $10") are not eaten: the span must stay
/// on one line, be non-empty, not be padded with whitespace, and the closing
/// delimiter must not be followed by a digit.
fn inline_math_end(body: &str) -> Option<usize> {
    let k = body.find('$')?;
    let content = &body[..k];
    if content.is_empty()
        || content.contains('\n')
        || content.starts_with(char::is_whitespace)
        || content.ends_with(char::is_whitespace)
        || body[k + 1..].starts_with(|c: char| c.is_ascii_digit())
    {
        return None;
    }
    Some(k)
}

/// Reduce a TeX math body to plain text: unwrap text-mode groups, map spacing
/// and common symbol commands, drop grouping braces, and collapse whitespace.
fn detex(tex: &str) -> String {
    let mut out = String::with_capacity(tex.len());
    let mut chars = tex.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                let mut cmd = String::new();
                while let Some(&n) = chars.peek() {
                    if n.is_ascii_alphabetic() {
                        cmd.push(n);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if cmd.is_empty() {
                    // Single-character controls: \, \; \: \  are spacing, \! is
                    // negative space, the rest (\$ \{ \} \% …) escape literals.
                    if let Some(n) = chars.next() {
                        match n {
                            ',' | ';' | ':' | ' ' => out.push(' '),
                            '!' => {}
                            _ => out.push(n),
                        }
                    }
                } else {
                    match cmd.as_str() {
                        "text" | "textbf" | "textit" | "textrm" | "texttt" | "mathrm"
                        | "mathbf" | "mathit" | "operatorname" | "mbox" => {
                            out.push_str(&detex(&take_braced_group(&mut chars)));
                        }
                        "quad" | "qquad" => out.push(' '),
                        "times" => out.push('×'),
                        "cdot" => out.push('·'),
                        "pm" => out.push('±'),
                        "rightarrow" | "to" => out.push('→'),
                        "Rightarrow" | "implies" => out.push('⇒'),
                        "leftarrow" => out.push('←'),
                        "ldots" | "dots" | "cdots" => out.push_str("..."),
                        "geq" | "ge" => out.push('≥'),
                        "leq" | "le" => out.push('≤'),
                        "neq" | "ne" => out.push('≠'),
                        "infty" => out.push('∞'),
                        // Unknown command: drop the backslash, keep the name.
                        _ => out.push_str(&cmd),
                    }
                }
            }
            '{' | '}' => {}
            '~' | '\n' => out.push(' '),
            _ => out.push(c),
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Consume a `{...}`-delimited group (brace-nesting aware) and return its
/// contents; returns empty if the next character is not `{`.
fn take_braced_group(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut inner = String::new();
    if chars.peek() != Some(&'{') {
        return inner;
    }
    chars.next();
    let mut depth = 1u32;
    for c in chars.by_ref() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        inner.push(c);
    }
    inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_gemini_display_math() {
        let input = r"White simply recaptures: $$\text{7. d4 \quad exd4 \quad 8. Nxd4!}$$";
        assert_eq!(
            ChaissApp::sanitize_markdown(input),
            "White simply recaptures: **7. d4 exd4 8. Nxd4!**"
        );
    }

    #[test]
    fn rewrites_inline_math() {
        assert_eq!(
            ChaissApp::sanitize_markdown(r"The reply $e5$ is forced."),
            "The reply **e5** is forced."
        );
    }

    #[test]
    fn rewrites_multiline_display_math() {
        let input = "Eval:\n$$\n\\text{8. Nxd4} \\quad \\pm 1.3\n$$\nWhite is better.";
        assert_eq!(
            ChaissApp::sanitize_markdown(input),
            "Eval:\n**8. Nxd4 ± 1.3**\nWhite is better."
        );
    }

    #[test]
    fn leaves_unterminated_stream_untouched() {
        let input = r"Recapture: $$\text{7. d4";
        assert_eq!(ChaissApp::sanitize_markdown(input), input);
    }

    #[test]
    fn leaves_prices_untouched() {
        let input = "That engine costs $5 and $10 more per month.";
        assert_eq!(ChaissApp::sanitize_markdown(input), input);
    }

    #[test]
    fn leaves_code_fences_untouched() {
        let input = "Run:\n```sh\necho $$PATH\n```\ndone $x$";
        assert_eq!(
            ChaissApp::sanitize_markdown(input),
            "Run:\n```sh\necho $$PATH\n```\ndone **x**"
        );
    }

    #[test]
    fn still_closes_unbalanced_fences() {
        assert_eq!(
            ChaissApp::sanitize_markdown("```rust\nlet x = 1;"),
            "```rust\nlet x = 1;\n```\n"
        );
    }

    #[test]
    fn handles_nested_text_groups_and_symbols() {
        assert_eq!(
            ChaissApp::sanitize_markdown(r"$$\text{Nf3 \textbf{best}} \rightarrow \infty$$"),
            "**Nf3 best → ∞**"
        );
    }
}
