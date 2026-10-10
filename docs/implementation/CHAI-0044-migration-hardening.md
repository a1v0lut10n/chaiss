# CHAI-0044 — tolerate newer databases and explain open failures

- **Ticket:** CHAI-0044
- **Branch:** `bugfix/CHAI-0044-migration-hardening`
- **Status:** implemented (PR pending review)

## Premise

Every chaiss launch runs the embedded sqlx migrations against
`chaiss.db`. Upgrading works: 0.4.0 adds `last_llm_target` to a file
from any earlier release. The reverse does not: a chaiss older than the
build that last migrated the file panics at startup with
`Migrate(VersionMissing(20260914000001))` behind the message "Failed to
securely bind SQLite Database locally!". That bites whenever one
database is reached by more than one chaiss version — a file synced
between machines, a downgrade after a bad release, a second checkout.
The panic names neither the file nor the cause.

Separately, a developer building with `DATABASE_URL` set (the `.env`
from `.env.example`) sees `no such column: last_llm_target` at *compile*
time when the local file has not been migrated yet, because the
`sqlx::query!` macros validate against the live database while the
migrations only run when the app starts. That is a setup gap, not a
code bug; it gets a README note.

## Action items

1. `DbClient::new` sets `ignore_missing` on the migrator: ledger rows
   for migrations this build does not know are skipped instead of
   fatal. Safe because migrations are additive-only, now recorded as
   ADR-0001. A known migration whose checksum differs from the applied
   one is still rejected (`VersionMismatch`).
2. `db::describe_open_failure(database_url, &sqlx::Error) -> String`
   renders a startup failure for the person at the keyboard: the file,
   the underlying error, and specific guidance for `VersionMismatch`,
   `VersionMissing`, and `Dirty`, plus the `DATABASE_URL` escape hatch.
   `main.rs` prints it and exits 1 instead of `expect`-panicking.
3. Migration tests in `chaiss-core` against real SQLite files in a
   scratch directory (no new dependencies): fresh file gets every
   shipped migration; a 0.1.x-shaped file (initial migration only) is
   upgraded in place; a file carrying a future ledger row still opens
   and round-trips `last_llm_target`; a tampered checksum is rejected
   and the guidance names the version. Tests use unchecked
   `sqlx::query` so the committed `.sqlx` cache needs no regeneration.
4. README developer note: with `DATABASE_URL` set, apply the
   migrations once (`sqlx migrate run --source chaiss-core/migrations`)
   or build with `SQLX_OFFLINE=true`; `.env.example` gains the matching
   commented line. Publish skill checklist gains the same reminder.
5. CHANGELOG `[Unreleased]` entry.

## Verification checklist

- [x] fmt / clippy `-D warnings` / `cargo test --workspace` clean.
- [x] v0.3.2-era scenario reproduced in test: a database carrying a
      ledger row this build does not know opens and is usable.
- [x] Manual: current build opens this machine's un-migrated
      `chaiss.db` copy (init-only ledger) and migrates it.

## Out of scope

- Protecting users still on 0.3.x: they only get the fix by upgrading.
- Non-additive migrations (drops, renames, type changes). ADR-0001
  forbids them; a future need for one requires a new decision.
- A CI gate that the crate-local `.sqlx` cache matches the migrations
  (candidate follow-up; see the journal entry).
