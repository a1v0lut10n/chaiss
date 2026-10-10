# CHAI-0045 — let `.chaiss.cola` configure the database location

- **Ticket:** CHAI-0045
- **Branch:** `feature/CHAI-0045-cola-database-location` (follows
  CHAI-0044, PR #60)
- **Status:** planned

## Premise

`.chaiss.cola` promises to take precedence over `.env` per key, and
every LLM key already does: `LLM_BACKEND`, the per-provider API keys
(with the `GEMINI_API_KEY` alias), and the per-provider models all
resolve through `cola_config::config_value`. The one `.env` key the
app reads outside that lookup is `DATABASE_URL`, taken straight from
the process environment in `main.rs`. CHAI-0044 documented that
asymmetry; this ticket removes it, so a cola-only setup can name its
database file without a `.env`.

What this ticket cannot align: `SQLX_OFFLINE` and the compile-time
`DATABASE_URL` that the `sqlx::query!` macros read. Those act at build
time, before any cola file is parsed, and stay an `.env`/environment
matter (README developer note). The cola value governs the file the
running app opens, nothing else.

## Shape

A top-level `database` entity beside `llm`, which the colap grammar
already accepts (verified: `find_entity_by_path("database")` and the
`url` field resolve on a two-entity document):

```cola
database:
    url: "sqlite:chaiss.db"
;
```

## Action items

1. `cola_config::overlay_from_str` maps `database.url` to the logical
   key `DATABASE_URL`, under the same placeholder rule as API keys (an
   empty value never enters the overlay; the example may ship the
   default without un-configuring anything).
2. `main.rs` resolves the database URL through
   `cola_config::config_value("DATABASE_URL")` with the existing
   `sqlite://chaiss.db` default. `cola_config::init` already runs
   before the database opens, so ordering holds; add a comment that it
   must stay that way.
3. `db::describe_open_failure`'s closing hint names both sources:
   `DATABASE_URL` in `.env`/the environment, or `database.url` in
   `.chaiss.cola`.
4. `.chaiss.cola.example`: add the `database` block to the fenced
   configuration with a short prose paragraph, and rewrite the
   CHAI-0044 Notes bullet (the file *does* configure the database
   location now; the build-time caveat remains `.env`-only). The
   example-parsing unit test keeps it honest.
5. README: the cola paragraph in Environment Configuration mentions
   the database location; the developer note's last sentence is
   reworded (cola sets the runtime file, not the build-time check).
6. Tests in `cola_config`: `database.url` maps to `DATABASE_URL`; an
   empty url is dropped; a document without a `database` entity leaves
   the key unset so `.env` still applies.
7. CHANGELOG `[Unreleased]` entry under Added.

## Verification checklist

- [ ] fmt / clippy `-D warnings` / `cargo test --workspace` clean.
- [ ] Manual: with no `.env` and a `.chaiss.cola` naming a scratch
      file, the app creates and migrates that file; with both files
      present, the cola value wins; with the cola block absent, `.env`
      applies.
- [ ] Plan updated to what was built.

## Out of scope

- Mirroring the legacy `LLM_MODEL` key: the per-provider `model`
  field already covers it and is the preferred form.
- Any build-time behaviour (`SQLX_OFFLINE`, macro-time `DATABASE_URL`).
- A non-SQLite database; `url` is passed through unchanged.
