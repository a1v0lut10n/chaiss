# Apache 2.0 license text and attribution

- **When:** 2026-10-19 17:03:32 local
- **Type:** execution
- **Project:** chaiss

## Context

The project had been published to GitHub and crates.io without a license
text or a copyright holder. Both manifests had declared
`license = "Apache-2.0"` since the first publish, so the registry
advertised a license whose text and attribution shipped nowhere.

## Details

Adopted Apache 2.0 explicitly: the canonical text from apache.org as
`LICENSE` (checksum-compared with the copy serde ships, then the
appendix's copyright placeholder expanded with the holder's details after
review caught it still in template form), a `NOTICE`
naming Hans Van Wesenbeeck as copyright holder for 2026, and a License
section in the README carrying the boilerplate notice and the Section 5
contribution clause. Cargo 1.99 does not include a workspace-root
`LICENSE` in a member package, so each crate directory symlinks both
files; `cargo package` dereferences them, verified by inspecting both
0.5.0 tarballs. Per-file headers were left out as not required.

Apache 2.0 over MIT was a deliberate choice for the explicit patent grant,
the built-in contribution terms and the trademark reservation; the costs
are the longer text, the NOTICE and changed-file obligations on
redistributors, and GPLv2 incompatibility. The Rust-conventional
`MIT OR Apache-2.0` dual form stays available as a one-line change.

## Links

- PR: https://github.com/a1v0lut10n/chaiss/pull/69
- Plan: `docs/implementation/CHAI-0049-apache-2-license.md`
