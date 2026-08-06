# Local NEXT-TICKET branch numbering replaces Jira CHAI series

- **When:** 2026-08-06 12:12:55 local
- **Type:** decision
- **Project:** chaiss

## Context

Branch numbers historically came from Jira (`CHAI-01`..`CHAI-30`). Adopted
brainforge-app's NEXT-TICKET approach: a committed counter file issues numbers
locally, and concurrent claims of the same number conflict at merge time
instead of colliding silently.

## Details

`docs/NEXT-TICKET` seeded with `CHAI-0031`; convention documented in
`AGENTS.md`. Numbers are four-digit zero-padded
(`feature/CHAI-0031-short-name`), claimed by incrementing the file in the
branch's first commit. Trivial housekeeping branches may go unnumbered.

## Links

- `AGENTS.md` — Branch Naming & Ticket Numbers
- Origin: brainforge-app `CLAUDE.md` (BF-NNNN / NEXT-TICKET)
