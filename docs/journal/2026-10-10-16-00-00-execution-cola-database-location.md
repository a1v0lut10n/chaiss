# Cola names the database file

- **When:** 2026-10-10 16:00:00 local
- **Type:** execution
- **Project:** chaiss

## Context

CHAI-0044 documented that `.chaiss.cola` carried LLM settings only,
leaving `DATABASE_URL` the one `.env` key outside the per-key
precedence promise. CHAI-0045 closes the gap.

## Details

A top-level `database.url` maps to `DATABASE_URL` in the overlay
(`cola_config::overlay_from_str`); `main.rs` resolves the url
through `config_value`, cola first and environment second, with the
unchanged `sqlite://chaiss.db` default. The example ships an empty
url, since shipping the default path would have masked a `.env`
value the moment the example was copied. The cola value governs the
running app only; the build-time sqlx query check still reads `.env`
and the environment, which the README and the example state. The
open-failure hint names both sources. Two new tests plus the
example-file assertion; manual checks from a scratch directory
covered cola-only, cola over environment, empty url, and the failure
hint. PR #62.

## Lesson

A per-key precedence promise is only honest when every key the app
reads goes through the same lookup; grep for direct environment reads
whenever a new setting is added.

## Links

- Plan: `docs/implementation/CHAI-0045-cola-database-location.md`
