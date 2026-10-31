//! Cola-based configuration (`.chaiss.cola`): LLM settings and the
//! database location.
//!
//! The workspace's cola configuration language (the `colap` crate) is
//! literate: a markdown document with fenced ```` ```cola ```` blocks
//! and a bare cola file read the same way through colap's facade, so
//! the literate `.chaiss.cola.example` is itself valid input.
//!
//! The file is read once at startup ([`init`]) into an overlay of the
//! same logical keys the `.env` configuration uses (`LLM_BACKEND`,
//! `GOOGLE_API_KEY`, `LLM_MODEL_OPENAI`, `DATABASE_URL`, ...). Lookups go through
//! [`config_value`]: overlay first, environment fallback — so a
//! `.chaiss.cola` takes precedence over `.env` per key when present.

use colap::config_model::{ConfigModel, ConfigValue};
use std::collections::HashMap;
use std::path::Path;
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
/// adds quotes, so the text kinds are taken as they are). A number is
/// its literal, kept exactly as written; a list is its elements joined
/// by spaces, the way an environment variable would carry them.
fn value_text(value: &ConfigValue) -> String {
    match value {
        ConfigValue::String(s) | ConfigValue::Symbol(s) | ConfigValue::Reference(s) => s.clone(),
        ConfigValue::Number(n) => n.text().to_string(),
        ConfigValue::Boolean(b) => b.to_string(),
        ConfigValue::List(items) => items.iter().map(value_text).collect::<Vec<_>>().join(" "),
    }
}

fn field_at(model: &ConfigModel, path: &str, field: &str) -> Option<String> {
    let entity = model.find_entity_by_path(path)?;
    model.get_field_value(entity, field).map(|v| value_text(&v))
}

/// Parses cola content (literate markdown or bare cola, as colap
/// detects it) into an overlay of the logical configuration keys the
/// `.env` lookup already uses. The error names the line and column.
pub fn overlay_from_str(content: &str) -> Result<HashMap<String, String>, String> {
    let parsed = colap::parse_str(content, None).map_err(|e| e.to_string())?;
    Ok(overlay_from_model(&parsed.model))
}

/// The overlay of a parsed configuration. Colap stores a
/// `provider plural providers` collection under its singular name, so
/// provider entries live at `llm/provider/<name>`.
fn overlay_from_model(model: &ConfigModel) -> HashMap<String, String> {
    // A placeholder or empty value never enters the overlay: it would
    // mask a real key set in `.env` (the example ships placeholders for
    // every provider, and copying it must not un-configure anything).
    let mut overlay = HashMap::new();
    let mut set = |key: String, value: String| {
        if !value.trim().is_empty() && value != "your_key_here" {
            overlay.insert(key, value);
        }
    };
    if let Some(backend) = field_at(model, "llm", "backend") {
        set("LLM_BACKEND".to_string(), backend);
    }
    for (provider, key_env) in PROVIDER_KEYS {
        let path = format!("llm/provider/{provider}");
        if let Some(api_key) = field_at(model, &path, "api_key") {
            if !key_env.is_empty() {
                set(key_env.to_string(), api_key);
            }
        }
        if let Some(model_name) = field_at(model, &path, "model") {
            set(format!("LLM_MODEL_{}", provider.to_uppercase()), model_name);
        }
    }
    // `database.url` names the file the running app opens. It cannot reach
    // the build-time sqlx query check, which reads `.env`/the environment
    // only; an empty value falls through to `.env` like a placeholder key.
    if let Some(url) = field_at(model, "database", "url") {
        set("DATABASE_URL".to_string(), url);
    }
    overlay
}

/// Loads `.chaiss.cola` from the working directory into the process-wide
/// overlay. Call once at startup, before the UI starts. A missing file
/// is normal (env-only configuration); a malformed file never crashes —
/// every finding is reported on the console as `path:line:col:
/// severity: message` and the overlay stays empty. Warnings (an
/// unclosed fence, an empty entity) are reported and the file is used.
pub fn init() {
    let overlay = if Path::new(COLA_CONFIG_FILE).is_file() {
        let report = colap::check_file(COLA_CONFIG_FILE);
        for finding in report.diagnostics() {
            eprintln!("[config] {finding}");
        }
        match report.outcome {
            Ok(model) => {
                let overlay = overlay_from_model(&model);
                println!(
                    "Loaded configuration from {COLA_CONFIG_FILE} ({} keys; takes precedence over .env).",
                    overlay.len()
                );
                overlay
            }
            Err(_) => {
                eprintln!(
                    "[config error] {COLA_CONFIG_FILE} is ignored because of the errors above (falling back to .env)."
                );
                HashMap::new()
            }
        }
    } else {
        HashMap::new()
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

    // A trailing comma before a nested entity has been optional since
    // colap 0.3; the example keeps the older spelling without it.
    const PLAIN: &str = r#"llm:
    backend: "openai"
    provider plural providers:
        google:
            api_key: "g-key",
            model: "gemini-3.8-flash"
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
        assert_eq!(overlay.get("LLM_MODEL_GOOGLE").unwrap(), "gemini-3.8-flash");
        assert_eq!(overlay.get("LLM_MODEL_OPENAI").unwrap(), "gpt-6-astra");
        assert!(!overlay.contains_key("ANTHROPIC_API_KEY"));
        // No `database` entity: the key stays unset so .env still applies.
        assert!(!overlay.contains_key("DATABASE_URL"));
    }

    #[test]
    fn database_url_maps_to_logical_key() {
        let content =
            format!("{PLAIN}database:\n    url: \"sqlite:///var/lib/chaiss/games.db\"\n;\n");
        let overlay = overlay_from_str(&content).unwrap();
        assert_eq!(
            overlay.get("DATABASE_URL").unwrap(),
            "sqlite:///var/lib/chaiss/games.db"
        );
        assert_eq!(overlay.get("LLM_BACKEND").unwrap(), "openai");
    }

    #[test]
    fn empty_database_url_falls_through_to_env() {
        let content = "database:\n    url: \"\"\n;\n";
        let overlay = overlay_from_str(content).unwrap();
        assert!(!overlay.contains_key("DATABASE_URL"));
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
    fn a_malformed_file_names_its_line() {
        let error = overlay_from_str("llm:\n    backend: \"openai\",\n    broken {\n;\n")
            .expect_err("malformed");
        assert!(error.starts_with("3:"), "{error}");
    }

    #[test]
    fn symbols_numbers_and_comments_are_read() {
        let content = "llm: // the backend\n    backend: openai, // a symbol\n;\ndatabase: url: 'sqlite:x', pool: 1_0;\n";
        let overlay = overlay_from_str(content).unwrap();
        assert_eq!(overlay.get("LLM_BACKEND").unwrap(), "openai");
        assert_eq!(overlay.get("DATABASE_URL").unwrap(), "sqlite:x");
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
        // The example ships an empty database url, so copying it never
        // masks a DATABASE_URL set in .env.
        assert!(!overlay.contains_key("DATABASE_URL"));
    }
}
