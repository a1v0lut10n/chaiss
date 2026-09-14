//! Cola-based LLM configuration (`.chaiss.cola`).
//!
//! The workspace's cola configuration language (the `colap` crate) is
//! markdown-native: plain cola files and markdown documents with fenced
//! ```` ```cola ```` blocks parse with the same parser, so a literate
//! config (see `.chaiss.cola.example`) is itself valid input.
//!
//! The file is read once at startup ([`init`]) into an overlay of the
//! same logical keys the `.env` configuration uses (`LLM_BACKEND`,
//! `GOOGLE_API_KEY`, `LLM_MODEL_OPENAI`, ...). Lookups go through
//! [`config_value`]: overlay first, environment fallback — so a
//! `.chaiss.cola` takes precedence over `.env` per key when present.

use colap::cola::ColaParser;
use colap::config_model::{ConfigModel, ConfigValue};
use colap::model_builder::ModelBuilder;
use rustemo::Parser;
use std::collections::HashMap;
use std::sync::OnceLock;

/// The config file read from the working directory, dotted like `.env`.
pub const COLA_CONFIG_FILE: &str = ".chaiss.cola";

/// Providers recognized in `.chaiss.cola`, with the env key their
/// `api_key` field maps to (empty for the keyless Ollama).
const PROVIDER_KEYS: [(&str, &str); 4] = [
    ("google", "GOOGLE_API_KEY"),
    ("openai", "OPENAI_API_KEY"),
    ("anthropic", "ANTHROPIC_API_KEY"),
    ("ollama", ""),
];

/// Overlay parsed once at startup; plain `Send + Sync` data, written
/// before the frame loop starts and read-only afterwards.
static OVERLAY: OnceLock<HashMap<String, String>> = OnceLock::new();

/// A field value as configuration text (colap's `Display` for strings
/// adds quotes, so unwrap the string variant explicitly).
fn value_text(value: &ConfigValue) -> String {
    match value {
        ConfigValue::String(s) => s.clone(),
        ConfigValue::Integer(i) => i.to_string(),
        ConfigValue::Float(f) => f.to_string(),
        ConfigValue::Boolean(b) => b.to_string(),
    }
}

fn field_at(model: &ConfigModel, path: &str, field: &str) -> Option<String> {
    let entity = model.find_entity_by_path(path)?;
    model.get_field_value(entity, field).map(|v| value_text(&v))
}

/// Parses cola content (plain or markdown-embedded) into an overlay of
/// the logical configuration keys the `.env` lookup already uses.
///
/// Colap's grammar is markdown-first: only fenced ```` ```cola ````
/// blocks carry configuration, and fence-less text parses as ignored
/// prose. Plain cola input is therefore wrapped in a synthetic fence
/// before parsing, so both forms work. Colap stores a
/// `provider plural providers` collection under its singular name, so
/// provider entries live at `llm/provider/<name>`.
pub fn overlay_from_str(content: &str) -> Result<HashMap<String, String>, String> {
    let fenced;
    let content = if content.contains("```cola") {
        content
    } else {
        fenced = format!("```cola\n{content}\n```\n");
        &fenced
    };
    let ast = ColaParser::new()
        .parse(content)
        .map_err(|e| e.to_string())?;
    let model = ModelBuilder::build_config_model(&ast)?;

    // A placeholder or empty value never enters the overlay: it would
    // mask a real key set in `.env` (the example ships placeholders for
    // every provider, and copying it must not un-configure anything).
    let mut overlay = HashMap::new();
    let mut set = |key: String, value: String| {
        if !value.trim().is_empty() && value != "your_key_here" {
            overlay.insert(key, value);
        }
    };
    if let Some(backend) = field_at(&model, "llm", "backend") {
        set("LLM_BACKEND".to_string(), backend);
    }
    for (provider, key_env) in PROVIDER_KEYS {
        let path = format!("llm/provider/{provider}");
        if let Some(api_key) = field_at(&model, &path, "api_key") {
            if !key_env.is_empty() {
                set(key_env.to_string(), api_key);
            }
        }
        if let Some(model_name) = field_at(&model, &path, "model") {
            set(format!("LLM_MODEL_{}", provider.to_uppercase()), model_name);
        }
    }
    Ok(overlay)
}

/// Loads `.chaiss.cola` from the working directory into the process-wide
/// overlay. Call once at startup, before the UI starts. A missing file
/// is normal (env-only configuration); a malformed file never crashes —
/// it is reported on the console and the overlay stays empty.
pub fn init() {
    let overlay = match std::fs::read_to_string(COLA_CONFIG_FILE) {
        Ok(content) => match overlay_from_str(&content) {
            Ok(overlay) => {
                println!(
                    "Loaded LLM configuration from {COLA_CONFIG_FILE} ({} keys; takes precedence over .env).",
                    overlay.len()
                );
                overlay
            }
            Err(e) => {
                eprintln!(
                    "[config error] {COLA_CONFIG_FILE} could not be parsed and is ignored (falling back to .env): {e}"
                );
                HashMap::new()
            }
        },
        Err(_) => HashMap::new(),
    };
    let _ = OVERLAY.set(overlay);
}

/// Layered configuration lookup: the `.chaiss.cola` overlay first, the
/// process environment second. Safe to call from any thread.
pub fn config_value(key: &str) -> Option<String> {
    if let Some(value) = OVERLAY.get().and_then(|overlay| overlay.get(key)) {
        return Some(value.clone());
    }
    std::env::var(key).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: no comma after `backend` — in cola, a comma continues a
    // field list, so a nested entity may not follow one.
    const PLAIN: &str = r#"llm:
    backend: "openai"
    provider plural providers:
        google:
            api_key: "g-key",
            model: "gemini-3.7-flash"
        ;
        openai:
            api_key: "o-key",
            model: "gpt-6-astra"
        ;
    ;
;
"#;

    #[test]
    fn maps_plain_cola_to_logical_keys() {
        let overlay = overlay_from_str(PLAIN).unwrap();
        assert_eq!(overlay.get("LLM_BACKEND").unwrap(), "openai");
        assert_eq!(overlay.get("GOOGLE_API_KEY").unwrap(), "g-key");
        assert_eq!(overlay.get("OPENAI_API_KEY").unwrap(), "o-key");
        assert_eq!(overlay.get("LLM_MODEL_GOOGLE").unwrap(), "gemini-3.7-flash");
        assert_eq!(overlay.get("LLM_MODEL_OPENAI").unwrap(), "gpt-6-astra");
        assert!(!overlay.contains_key("ANTHROPIC_API_KEY"));
    }

    #[test]
    fn parses_markdown_embedded_cola() {
        let md =
            format!("# My literate config\n\nSome prose.\n\n```cola\n{PLAIN}```\n\nMore prose.\n");
        let overlay = overlay_from_str(&md).unwrap();
        assert_eq!(overlay.get("OPENAI_API_KEY").unwrap(), "o-key");
        assert_eq!(overlay.get("LLM_BACKEND").unwrap(), "openai");
    }

    #[test]
    fn partial_config_yields_partial_overlay() {
        let content = r#"llm:
    provider plural providers:
        openai:
            api_key: "o-key"
        ;
    ;
;
"#;
        let overlay = overlay_from_str(content).unwrap();
        assert_eq!(overlay.get("OPENAI_API_KEY").unwrap(), "o-key");
        assert!(!overlay.contains_key("LLM_BACKEND"));
        assert!(!overlay.contains_key("LLM_MODEL_OPENAI"));
    }

    #[test]
    fn placeholder_values_never_enter_the_overlay() {
        let content = r#"llm:
    provider plural providers:
        google:
            api_key: "your_key_here"
        ;
        openai:
            api_key: "o-key"
        ;
    ;
;
"#;
        let overlay = overlay_from_str(content).unwrap();
        // A placeholder must fall through to .env instead of masking it.
        assert!(!overlay.contains_key("GOOGLE_API_KEY"));
        assert_eq!(overlay.get("OPENAI_API_KEY").unwrap(), "o-key");
    }

    #[test]
    fn malformed_cola_is_an_error_not_a_panic() {
        assert!(overlay_from_str("llm: this is { not cola").is_err());
    }

    #[test]
    fn example_file_is_a_valid_literate_config() {
        let example = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../.chaiss.cola.example"
        ))
        .expect("read .chaiss.cola.example");
        let overlay = overlay_from_str(&example).unwrap();
        // The example ships the placeholder for every keyed provider;
        // placeholders never enter the overlay (they would mask real
        // keys from .env), so no *_API_KEY key may be present, while
        // backend and the default model names must be.
        assert_eq!(overlay.get("LLM_BACKEND").unwrap(), "google");
        for key in ["GOOGLE_API_KEY", "OPENAI_API_KEY", "ANTHROPIC_API_KEY"] {
            assert!(!overlay.contains_key(key));
        }
        assert_eq!(
            overlay.get("LLM_MODEL_GOOGLE").unwrap(),
            crate::llm::DEFAULT_GOOGLE_MODEL
        );
        assert_eq!(
            overlay.get("LLM_MODEL_OPENAI").unwrap(),
            crate::llm::DEFAULT_OPENAI_MODEL
        );
    }
}
