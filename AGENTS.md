# AI Agent Guidelines

Welcome, AI Agents! When working on the Chaiss repository, please strictly adhere to the following rules:

## Code Quality & Verification
- **Formatting**: Always run `cargo fmt` before committing any Rust code to ensure stylistic consistency across the codebase.
- **Linting**: Always run `cargo clippy` and address any lints or warnings before wrapping up your task.
- **Build Checks**: Always run `cargo check` or `cargo build` to ensure no compiler errors are introduced.

## Branch Naming & Ticket Numbers
- Branches are named `<type>/CHAI-NNNN-short-name`, where `<type>` is one of
  `feature`, `bugfix`, `chore`, or `docs` (e.g. `feature/CHAI-0031-opening-book`).
- Ticket numbers are four digits, zero-padded, and issued locally — not by Jira.
  `docs/NEXT-TICKET` holds the next free number. To claim it: use it, and
  increment the file **in the first commit on the new branch**. Never pick a
  number by scanning branches or history — two branches claiming the same
  number will conflict on this file at merge time, which is exactly the alarm
  we want.
- Trivial housekeeping branches (e.g. `chore/release-x-y-z-followup`) may skip
  the number; anything with a scoped piece of work gets one.
- Historical branches `CHAI-01`..`CHAI-30` used Jira-issued two-digit numbers;
  the local series continues from `CHAI-0031`.
