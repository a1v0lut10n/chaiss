# ADR-0001 — additive-only migrations; older builds tolerate newer databases

- **Status:** accepted (2026-10-10)
- **Ticket:** CHAI-0044

## Context

chaiss embeds its sqlx migrations and applies them on every launch.
`chaiss.db` is a plain file next to the working directory, so one file
is routinely opened by more than one chaiss version: a synced checkout
on two machines, a downgrade after a bad release, a crates.io install
beside a development build. sqlx's default refuses to open a database
whose migration ledger holds a version the running build does not
know (`VersionMissing`), which turned a harmless additive column into a
startup panic for every older build.

## Decision

1. Migrations are **additive-only**: new tables, new nullable columns,
   new columns with a default. No drops, renames, type changes, or
   data rewrites that an older build would misread. A shipped migration
   file is never edited; corrections ship as a new migration.
2. Because of (1), `DbClient::new` runs the migrator with
   `ignore_missing` set: ledger rows for unknown migrations are skipped,
   so an older build opens a newer file and simply does not use the
   columns it does not know.
3. A migration the build *does* know but whose recorded checksum
   differs (`VersionMismatch`) stays fatal, with guidance naming the
   file and the version. That state means two diverging copies of the
   same migration, which no automatic choice can resolve safely.

## Consequences

- Schema changes stay cheap and safe across versions; the burden moves
  to the author of a migration to keep it additive.
- A future non-additive change needs a new ADR and a deliberate
  mechanism (a schema-version floor, or a one-way "upgraded by" marker)
  before it can ship.
- Older builds keep writing rows that leave newer columns at their
  defaults; newer code must already treat `NULL`/default as "unset",
  which the existing `flip_board` and `last_llm_target` readers do.
