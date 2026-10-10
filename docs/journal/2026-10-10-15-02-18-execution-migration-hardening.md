# Migration hardening: tolerate newer databases

- **When:** 2026-10-10 15:02:18 local
- **Type:** execution
- **Project:** chaiss

## Context

A `chaiss.db` last migrated by 0.4.0 on one machine could not be
opened by an older build elsewhere: sqlx refuses a migration-ledger
version it does not know (`VersionMissing`), and `main.rs` panicked
behind "Failed to securely bind SQLite Database locally!", naming
neither the file nor the cause. The same investigation found the
local compile-time trap: with `DATABASE_URL` set from `.env`, the
`sqlx::query!` macros validate against the live file, which only the
running app migrates, so a fresh clone fails to build with
`no such column`. The migration itself was never missing — it is in
the v0.4.0 tag and in the published crate.

## Details

CHAI-0044. `DbClient::new` sets ignore-missing on the migrator,
backed by ADR-0001 (migrations additive-only; shipped migrations never
edited; a checksum mismatch stays fatal). `db::describe_open_failure`
gives the file, the cause and guidance for `VersionMismatch`,
`VersionMissing` and `Dirty`; the app prints it and exits 1 instead
of panicking. Five migration tests run against real SQLite files in a
scratch directory: fresh file, 0.1.x-shaped file upgraded in place,
newer-than-this-build file, tampered checksum, and the guidance text.
They use unchecked queries, so the committed `.sqlx` cache needed no
regeneration. README, `.env.example` and the publish skill document
the compile-time trap.

The cola overlay was checked and documented as LLM-only: it cannot
carry `DATABASE_URL` or `SQLX_OFFLINE`, so the compile-time trap
exists only via `.env`, and the runtime hardening applies whichever
file configures the model. Aligning `.chaiss.cola` with `.env` on
the database location is the follow-up, CHAI-0045.

## Lesson

Forward tolerance has to be in the binary before it is needed; it
cannot be published retroactively, so users on 0.3.x only get it by
upgrading. A CI check that the crate-local `.sqlx` cache matches the
migrations, plus a dry-run publish of both crates, would catch the
shipping-side half of this class of problem.

## Links

- Plan: `docs/implementation/CHAI-0044-migration-hardening.md`
- Decision: `docs/decisions/ADR-0001-additive-migrations-forward-tolerance.md`
