---
name: publish-to-crates-io
description: Publish the chaiss / chaiss-core crates to crates.io. Use whenever releasing a new version to crates.io, running cargo publish, or preparing a release. Covers the mandatory SQLX_OFFLINE=true caveat and the two-crate publish order.
---

# Publishing chaiss to crates.io

The workspace has two publishable crates. `chaiss` depends on `chaiss-core`
(`chaiss-core = { version = "x.y.z", path = "../chaiss-core" }`), so they must be
published **in dependency order**:

1. `chaiss-core` first
2. `chaiss` second (only after the new `chaiss-core` version is live on crates.io)

## ⚠️ The SQLX_OFFLINE caveat (the important one)

`chaiss-core` uses compile-time-checked `sqlx::query!` / `sqlx::migrate!` macros
(`chaiss-core/src/db.rs`). During `cargo publish`, the crate is built in an
isolated tarball with **no `DATABASE_URL` and no live database**. Without help,
that build fails because the query macros try to connect to a DB.

The fix: build against the committed offline query cache instead. sqlx does this
when `SQLX_OFFLINE=true` is set, using the crate-local **`chaiss-core/.sqlx/`**
cache (which is committed to git — do not gitignore it).

```bash
# From the workspace root:
SQLX_OFFLINE=true cargo publish -p chaiss-core
# then, once chaiss-core is live on crates.io:
SQLX_OFFLINE=true cargo publish -p chaiss
```

> Note: the env var is **`SQLX_OFFLINE`** (not `SQLX_CACHE_OFFLINE`). The CI
> workflow (`.github/workflows/ci.yml`) already sets `SQLX_OFFLINE: true` for the
> same reason. `SQLX_OFFLINE_DIR` can override the cache location but is not
> needed here.

## Pre-publish checklist

1. Bump the version in **both** `chaiss-core/Cargo.toml` and `chaiss/Cargo.toml`,
   and the `chaiss-core` path-dependency `version` in `chaiss/Cargo.toml`. Run
   `cargo check` to refresh `Cargo.lock`.
2. Ensure `chaiss-core/.sqlx/` and `chaiss-core/migrations/` are committed and
   current. If any SQL query changed, regenerate the cache:
   `DATABASE_URL=sqlite:chaiss.db cargo sqlx prepare -p chaiss-core` (or
   `cargo sqlx prepare --workspace`), then commit the updated `.sqlx/`.
3. Dry-run both packages before the real publish:
   ```bash
   SQLX_OFFLINE=true cargo publish -p chaiss-core --dry-run
   SQLX_OFFLINE=true cargo publish -p chaiss --dry-run   # may complain the new chaiss-core isn't on crates.io yet — that's expected pre-publish
   ```
4. Publish in order (commands above).
5. Tag the release as an **annotated** tag matching the existing convention:
   `git tag -a vX.Y.Z -m "Release X.Y.Z: <short description>"` then
   `git push origin vX.Y.Z`.
