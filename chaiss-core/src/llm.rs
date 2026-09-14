use flume::Sender;
use futures::StreamExt;
use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::{ChatMessage, ReasoningEffort},
};

#[derive(Clone, Debug)]
pub struct LlmPromptPayload {
    pub prompt: String,
    pub current_fen: String,
    pub ascii_board: String,
    pub algebraic_history: Vec<String>,
    pub chat_history: Vec<(String, String)>,
    pub predictive_matrix_hotspots: Vec<String>,
    pub system_role: String,
    /// Which provider/model serves this request. API keys are never carried
    /// here — they are read from the environment at request time.
    pub target: LlmTarget,
}

/// Default model per backend when no `LLM_MODEL`/`LLM_MODEL_<PROVIDER>`
/// override is set.
pub const DEFAULT_GOOGLE_MODEL: &str = "gemini-3.7-flash";
pub const DEFAULT_OPENAI_MODEL: &str = "gpt-6-astra";
pub const DEFAULT_ANTHROPIC_MODEL: &str = "claude-3-opus-20240229";
pub const DEFAULT_OLLAMA_MODEL: &str = "llama3";

/// The LLM providers chaiss can talk to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LlmProvider {
    Google,
    OpenAI,
    Anthropic,
    Ollama,
}

const ALL_PROVIDERS: [LlmProvider; 4] = [
    LlmProvider::Google,
    LlmProvider::OpenAI,
    LlmProvider::Anthropic,
    LlmProvider::Ollama,
];

impl LlmProvider {
    pub fn display_name(self) -> &'static str {
        match self {
            LlmProvider::Google => "Google",
            LlmProvider::OpenAI => "OpenAI",
            LlmProvider::Anthropic => "Anthropic",
            LlmProvider::Ollama => "Ollama",
        }
    }

    /// The token used both as an `LLM_BACKEND` value and, uppercased, as the
    /// `LLM_MODEL_<PROVIDER>` suffix.
    fn env_token(self) -> &'static str {
        match self {
            LlmProvider::Google => "google",
            LlmProvider::OpenAI => "openai",
            LlmProvider::Anthropic => "anthropic",
            LlmProvider::Ollama => "ollama",
        }
    }

    fn api_key_env(self) -> &'static str {
        match self {
            LlmProvider::Google => "GOOGLE_API_KEY",
            LlmProvider::OpenAI => "OPENAI_API_KEY",
            LlmProvider::Anthropic => "ANTHROPIC_API_KEY",
            LlmProvider::Ollama => "",
        }
    }

    fn default_model(self) -> &'static str {
        match self {
            LlmProvider::Google => DEFAULT_GOOGLE_MODEL,
            LlmProvider::OpenAI => DEFAULT_OPENAI_MODEL,
            LlmProvider::Anthropic => DEFAULT_ANTHROPIC_MODEL,
            LlmProvider::Ollama => DEFAULT_OLLAMA_MODEL,
        }
    }
}

/// One selectable provider/model pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LlmTarget {
    pub provider: LlmProvider,
    pub model: String,
}

impl std::fmt::Display for LlmTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.model, self.provider.display_name())
    }
}

/// An environment lookup, abstracted so resolution stays a pure function and
/// tests never mutate process-global env vars.
type EnvLookup<'a> = &'a dyn Fn(&str) -> Option<String>;

/// A set env var counts as configured only when it holds a real value — the
/// `.env.example` placeholder is treated as unset.
fn env_value(lookup: EnvLookup, key: &str) -> Option<String> {
    lookup(key)
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty() && v != "your_key_here")
}

fn backend_token(lookup: EnvLookup) -> String {
    env_value(lookup, "LLM_BACKEND").map_or_else(|| "google".to_string(), |v| v.to_lowercase())
}

/// Model for `provider`: `LLM_MODEL_<PROVIDER>` override, else the legacy
/// `LLM_MODEL` when `LLM_BACKEND` selects this provider, else the default.
fn resolve_model(lookup: EnvLookup, provider: LlmProvider, backend: &str) -> String {
    let per_provider = format!("LLM_MODEL_{}", provider.env_token().to_uppercase());
    if let Some(model) = env_value(lookup, &per_provider) {
        return model;
    }
    if backend == provider.env_token() {
        if let Some(model) = env_value(lookup, "LLM_MODEL") {
            return model;
        }
    }
    provider.default_model().to_string()
}

fn configured_targets_from(lookup: EnvLookup) -> Vec<LlmTarget> {
    let backend = backend_token(lookup);
    ALL_PROVIDERS
        .iter()
        .filter(|&&provider| match provider {
            // Ollama is keyless: only offered when explicitly selected.
            LlmProvider::Ollama => backend == "ollama",
            LlmProvider::Google => {
                env_value(lookup, "GOOGLE_API_KEY").is_some()
                    || env_value(lookup, "GEMINI_API_KEY").is_some()
            }
            _ => env_value(lookup, provider.api_key_env()).is_some(),
        })
        .map(|&provider| LlmTarget {
            provider,
            model: resolve_model(lookup, provider, &backend),
        })
        .collect()
}

fn env_default_target_from(lookup: EnvLookup) -> LlmTarget {
    let backend = backend_token(lookup);
    let provider = ALL_PROVIDERS
        .iter()
        .copied()
        .find(|p| p.env_token() == backend)
        .unwrap_or(LlmProvider::Google);
    LlmTarget {
        provider,
        model: resolve_model(lookup, provider, &backend),
    }
}

/// All providers with an API key configured in the environment, in a stable
/// order, each with its resolved model.
pub fn configured_targets() -> Vec<LlmTarget> {
    configured_targets_from(&|key| std::env::var(key).ok())
}

/// The target `LLM_BACKEND` selects (google by default), regardless of
/// whether its API key is present — the missing key surfaces as a chat
/// error at request time, exactly as before.
pub fn env_default_target() -> LlmTarget {
    env_default_target_from(&|key| std::env::var(key).ok())
}

/// An LLM failure split into what the user should read and what a developer
/// needs for troubleshooting.
///
/// `user_message` is short, free of protocol jargon, and safe to render in the
/// chat panel. `detail` carries the raw backend error (status, JSON body, ...)
/// and is meant for the console log only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LlmError {
    pub user_message: String,
    pub detail: String,
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.user_message)
    }
}

impl std::error::Error for LlmError {}

impl LlmError {
    fn new(user_message: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            user_message: user_message.into(),
            detail: detail.into(),
        }
    }
}

/// Extract the first HTTP status code mentioned in a raw backend error, e.g.
/// `"... returned error status: 503 Service Unavailable ..."` or a JSON body
/// containing `"code": 503`.
fn extract_http_status(raw: &str) -> Option<u16> {
    let lower = raw.to_ascii_lowercase();
    for marker in [
        "error status: ",
        "\"code\": ",
        "\"code\":",
        "status code ",
        "status: ",
    ] {
        if let Some(pos) = lower.find(marker) {
            let digits: String = lower[pos + marker.len()..]
                .trim_start()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if digits.len() == 3 {
                if let Ok(code) = digits.parse::<u16>() {
                    return Some(code);
                }
            }
        }
    }
    None
}

/// Translate a raw backend/transport error into an [`LlmError`] whose
/// `user_message` explains what happened in plain language while `detail`
/// preserves the original text for the console.
pub fn classify_backend_error(provider: &str, model: &str, raw: impl Into<String>) -> LlmError {
    let raw = raw.into();
    let lower = raw.to_ascii_lowercase();
    let target = format!("{model} ({provider})");
    let status = extract_http_status(&raw);

    let overloaded = status == Some(503)
        || lower.contains("unavailable")
        || lower.contains("high demand")
        || lower.contains("overloaded");
    let rate_limited = status == Some(429)
        || lower.contains("resource_exhausted")
        || lower.contains("rate limit")
        || lower.contains("quota");
    let unauthorized = matches!(status, Some(401) | Some(403))
        || lower.contains("api key not valid")
        || lower.contains("permission_denied")
        || lower.contains("unauthenticated");
    let not_found = status == Some(404) || lower.contains("not_found");
    let server_error =
        matches!(status, Some(500) | Some(502) | Some(504)) || lower.contains("internal error");
    let network = lower.contains("timed out")
        || lower.contains("timeout")
        || lower.contains("connection")
        || lower.contains("dns")
        || lower.contains("network");

    let user_message = if overloaded {
        format!(
            "The AI backend {target} is temporarily unavailable due to high demand. \
             Spikes are usually short-lived — please try again in a moment."
        )
    } else if rate_limited {
        format!(
            "The AI backend {target} is rate-limiting requests or your quota is exhausted. \
             Please wait a little before retrying, or check your plan and billing."
        )
    } else if unauthorized {
        format!(
            "The AI backend {target} rejected the API key. \
             Please check the key in your `.env` file and restart the application."
        )
    } else if not_found {
        format!(
            "The model {target} was not found. \
             Please check `LLM_MODEL` in your `.env` file — the model may have been renamed or retired."
        )
    } else if server_error {
        format!(
            "The AI backend {target} reported an internal error. \
             This is on the provider's side — please try again shortly."
        )
    } else if network {
        format!(
            "Could not reach the AI backend {target}. \
             Please check your network connection and try again."
        )
    } else {
        format!(
            "The AI backend {target} returned an unexpected error. See the console for details."
        )
    };

    LlmError::new(user_message, raw)
}

// Orchestrate non-blocking HTTP REST streaming against the payload's target backend.
pub async fn stream_llm_response(
    payload: LlmPromptPayload,
    tx: Sender<String>,
) -> Result<(), LlmError> {
    let provider = payload.target.provider;
    let provider_name = provider.display_name();
    let api_key_env = provider.api_key_env();
    let model = payload.target.model.clone();

    let backend_enum = match provider {
        LlmProvider::OpenAI => LLMBackend::OpenAI,
        LlmProvider::Anthropic => LLMBackend::Anthropic,
        LlmProvider::Ollama => LLMBackend::Ollama, // Local testing fallback
        LlmProvider::Google => LLMBackend::Google,
    };

    // Google publishes the key as GEMINI_API_KEY in most of its docs; accept both spellings.
    let api_key = std::env::var(api_key_env)
        .or_else(|_| {
            if provider == LlmProvider::Google {
                std::env::var("GEMINI_API_KEY")
            } else {
                Err(std::env::VarError::NotPresent)
            }
        })
        .unwrap_or_else(|_| "TESTKEY".to_string());

    // Validate we actually have a key mapped, else error gracefully without crashing!
    // Skip key check for Ollama / Local backends
    if api_key == "TESTKEY" && provider != LlmProvider::Ollama {
        return Err(LlmError::new(
            format!(
                "No API key found for the {provider_name} backend. \
                 Please set `{api_key_env}` in your `.env` file and restart the application."
            ),
            format!(
                "environment variable {api_key_env} is not set (target: {})",
                payload.target
            ),
        ));
    }

    let mut builder = LLMBuilder::new()
        .api_key(api_key.clone())
        .model(&model)
        .max_tokens(8000);

    // OpenAI reasoning models (GPT-6 Astra and kin) reject any non-default
    // temperature with a 400; leaving it unset omits the field entirely.
    if provider != LlmProvider::OpenAI {
        builder = builder.temperature(0.7);
    }

    if provider == LlmProvider::Google {
        builder = builder
            .backend(LLMBackend::OpenAI)
            .base_url("https://generativelanguage.googleapis.com/v1beta/openai/")
            .reasoning_effort(ReasoningEffort::High);
    } else {
        builder = builder.backend(backend_enum);
    }

    let llm = builder
        .build()
        .map_err(|e| {
            LlmError::new(
                format!("Could not initialise the {provider_name} client for model {model}. See the console for details."),
                format!("Failed LLM Build: {e:?}"),
            )
        })?;

    let fen_parts: Vec<&str> = payload.current_fen.split_whitespace().collect();
    let is_white_turn = fen_parts.get(1).is_none_or(|&p| p == "w");
    let active_color = if is_white_turn { "WHITE" } else { "BLACK" };

    // 1. Build context mathematically formatting history and explicit ASCII layouts securely
    let formatted_history: String = payload
        .algebraic_history
        .iter()
        .enumerate()
        .map(|(i, mov)| format!("{}. {}", i, mov))
        .collect::<Vec<_>>()
        .join("\n");
    let mut futuristic_foresight = String::new();
    if !payload.predictive_matrix_hotspots.is_empty() {
        futuristic_foresight = format!(
            "\n\nCRITICAL CONTEXT INJECTION:\nThe Rust Engine's 2nd-Order Predictive Matrix natively resolved that the following squares will become the MOST densely contested structural targets 1-ply into the future: {}\nIncorporate this absolute mathematical foresight organically into your conceptual strategic analysis!",
            payload.predictive_matrix_hotspots.join(", ")
        );
    }

    let system_prompt = format!(
        "You are Chaiss, an advanced Chess {} mathematically bound to geometrical analysis.\n\n\
Current FEN String:\n{}\n\n\
Structural ASCII Board Matrix:\n{}\n\n\
Full Explicit Match Algebraic Sequence:\n{}\n\n\
The geometry currently dictates it is {}'s turn to move. \
Critically evaluate physical piece interactions natively, recognize structural blunders explicitly, and predict future hostile pressure correctly. Focus your analysis purely geometrically tracking explicit pawn structure and piece coordination sequentially over time. The user provides algebraic prompts.{}\n\n\
FORMATTING CONSTRAINT: Respond in plain Markdown only. NEVER use LaTeX or math notation — no $ or $$ delimiters and no backslash commands such as \\text or \\quad. Write chess moves as plain standard algebraic notation text.\n\n\
CRITICALLY BINDING REQUIREMENT: At the mathematical conclusion of your analysis, you MUST provide exactly one hypothesized continuation line up to 4 plies deep recursively, formatted distinctly exactly on a single line like this:\n\
### PREDICTIVE MATRIX: e4, e5, Nf3, Nc6",
        payload.system_role,
        payload.current_fen,
        payload.ascii_board,
        formatted_history,
        active_color,
        futuristic_foresight
    );

    // 2. Synthesize Context Matrix iteratively mimicking continuous API session strings cleanly
    let mut messages = vec![ChatMessage::user().content(&system_prompt).build()];

    // Inject a dummy acknowledgment so Gemini mathematically anchors the System constraints before our formal chat!
    messages.push(ChatMessage::assistant().content("System Context Acknowledged. I am mathematically bound to the supplied FEN bounds.").build());

    for (role, content) in payload.chat_history {
        if role == "User" {
            messages.push(ChatMessage::user().content(&content).build());
        } else {
            messages.push(ChatMessage::assistant().content(&content).build());
        }
    }

    // Inject the final active mathematical Prompt
    messages.push(ChatMessage::user().content(&payload.prompt).build());

    let mut stream = llm.chat_stream(&messages).await.map_err(|e| {
        classify_backend_error(provider_name, &model, format!("Chat Stream err: {e}"))
    })?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(token) => {
                let _ = tx.send_async(token).await;
            }
            Err(e) => {
                return Err(classify_backend_error(
                    provider_name,
                    &model,
                    format!("Network Stream Disconnected Abruptly: {e}"),
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const GEMINI_503: &str = "Chat Stream err: Response Format Error: OpenAI API returned error status: 503 Service Unavailable. Raw response: [{ \"error\": { \"code\": 503, \"message\": \"This model is currently experiencing high demand. Spikes in demand are usually temporary. Please try again later.\", \"status\": \"UNAVAILABLE\" } } ]";

    fn env(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn targets(map: &HashMap<String, String>) -> Vec<LlmTarget> {
        configured_targets_from(&|key| map.get(key).cloned())
    }

    #[test]
    fn enumerates_targets_from_configured_keys() {
        let map = env(&[("GOOGLE_API_KEY", "g-key"), ("OPENAI_API_KEY", "o-key")]);
        assert_eq!(
            targets(&map),
            vec![
                LlmTarget {
                    provider: LlmProvider::Google,
                    model: DEFAULT_GOOGLE_MODEL.to_string(),
                },
                LlmTarget {
                    provider: LlmProvider::OpenAI,
                    model: "gpt-6-astra".to_string(),
                },
            ]
        );
    }

    #[test]
    fn accepts_gemini_key_alias_and_placeholder_is_unset() {
        let map = env(&[
            ("GEMINI_API_KEY", "g-key"),
            ("OPENAI_API_KEY", "your_key_here"),
            ("ANTHROPIC_API_KEY", "  "),
        ]);
        let found = targets(&map);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].provider, LlmProvider::Google);
    }

    #[test]
    fn legacy_llm_model_applies_only_to_active_backend() {
        let map = env(&[
            ("GOOGLE_API_KEY", "g-key"),
            ("OPENAI_API_KEY", "o-key"),
            ("LLM_MODEL", "gemini-experimental"),
        ]);
        let found = targets(&map);
        assert_eq!(found[0].model, "gemini-experimental"); // LLM_BACKEND defaults to google
        assert_eq!(found[1].model, DEFAULT_OPENAI_MODEL);
    }

    #[test]
    fn per_provider_override_beats_legacy_llm_model() {
        let map = env(&[
            ("OPENAI_API_KEY", "o-key"),
            ("LLM_BACKEND", "openai"),
            ("LLM_MODEL", "gpt-5.6-sol"),
            ("LLM_MODEL_OPENAI", "gpt-6-astra-mini"),
        ]);
        assert_eq!(targets(&map)[0].model, "gpt-6-astra-mini");
    }

    #[test]
    fn ollama_offered_only_when_backend_selected() {
        assert!(targets(&env(&[("GOOGLE_API_KEY", "g")]))
            .iter()
            .all(|t| t.provider != LlmProvider::Ollama));
        let map = env(&[("LLM_BACKEND", "ollama")]);
        assert_eq!(
            targets(&map),
            vec![LlmTarget {
                provider: LlmProvider::Ollama,
                model: DEFAULT_OLLAMA_MODEL.to_string(),
            }]
        );
    }

    #[test]
    fn env_default_target_honors_backend_and_falls_back_to_google() {
        let map = env(&[("LLM_BACKEND", "anthropic")]);
        let target = env_default_target_from(&|key| map.get(key).cloned());
        assert_eq!(target.provider, LlmProvider::Anthropic);

        let empty = env(&[]);
        let target = env_default_target_from(&|key| empty.get(key).cloned());
        assert_eq!(target.provider, LlmProvider::Google);
        assert_eq!(target.model, DEFAULT_GOOGLE_MODEL);
    }

    #[test]
    fn target_display_matches_error_message_style() {
        let target = LlmTarget {
            provider: LlmProvider::OpenAI,
            model: "gpt-6-astra".to_string(),
        };
        assert_eq!(target.to_string(), "gpt-6-astra (OpenAI)");
    }

    #[test]
    fn extracts_status_from_crate_error_text() {
        assert_eq!(extract_http_status(GEMINI_503), Some(503));
        assert_eq!(
            extract_http_status("OpenAI API returned error status: 429 Too Many Requests"),
            Some(429)
        );
        assert_eq!(extract_http_status("{ \"code\": 404 }"), Some(404));
        assert_eq!(extract_http_status("connection reset by peer"), None);
    }

    #[test]
    fn gemini_high_demand_maps_to_friendly_message_and_keeps_detail() {
        let err = classify_backend_error("Google", "gemini-3.7-flash", GEMINI_503);
        assert!(err.user_message.contains("temporarily unavailable"));
        assert!(err.user_message.contains("gemini-3.7-flash (Google)"));
        assert!(!err.user_message.contains("503"));
        assert_eq!(err.detail, GEMINI_503);
        assert_eq!(err.to_string(), err.user_message);
    }

    #[test]
    fn classifies_other_statuses() {
        let m = |raw: &str| classify_backend_error("Google", "m", raw).user_message;
        assert!(m("error status: 429 Too Many Requests").contains("rate-limiting"));
        assert!(m("error status: 401 Unauthorized").contains("rejected the API key"));
        assert!(m("error status: 403 Forbidden").contains("rejected the API key"));
        assert!(m("error status: 404 Not Found").contains("was not found"));
        assert!(m("error status: 500 Internal Server Error").contains("internal error"));
        assert!(m("error status: 502 Bad Gateway").contains("internal error"));
        assert!(m("request timed out").contains("Could not reach"));
        assert!(m("something entirely different").contains("unexpected error"));
    }
}
