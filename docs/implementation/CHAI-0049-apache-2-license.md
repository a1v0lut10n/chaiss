# CHAI-0049 — license and attribution

- **Ticket:** CHAI-0049
- **Branch:** `chore/CHAI-0049-apache-2-license`
- **Status:** implemented (PR pending review)

## Premise

Both crates have carried `license = "Apache-2.0"` in their manifests since
the first crates.io publish, so the registry already advertises Apache 2.0,
but the repository holds no license text, no copyright notice and no NOTICE
file, and nothing in the README says who holds the copyright. The packaged
crates shipped the same way.

## Action items

1. `LICENSE`: the canonical Apache License 2.0 text from apache.org, with
   the appendix's `Copyright [yyyy] [name of copyright owner]` placeholder
   expanded to `Copyright 2026 Hans Van Wesenbeeck`; everything else
   unchanged.
2. `NOTICE`: project name and "Copyright 2026 Hans Van Wesenbeeck" (the
   first commit is from March 2026).
3. README: a License section with the standard boilerplate notice and the
   Section 5 contribution clause.
4. Ship the files in the crates. Cargo (1.99) does not pull a
   workspace-root `LICENSE` into a member package, so each crate directory
   holds `LICENSE` and `NOTICE` symlinks to the root files; `cargo package`
   dereferences them, and both 0.5.0 tarballs were checked to contain the
   full text.

## Out of scope

- Per-file SPDX / copyright headers. Apache 2.0 does not require them;
  they can be added later if wanted.
- A new crates.io release; the files reach the registry with the next
  version bump.
