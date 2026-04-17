# Chaiss Publication Plan

Restructure the Cargo workspace to map perfectly into an intuitive end-user deployment logic via `crates.io`, specifically prioritizing the desktop application as the primary `chaiss` interface. 

## User Review Required

> [!WARNING]
> Renaming the underlying crate directories and updating Cargo namespaces inherently triggers a large git diff because folders literally physically move. This also means any open configuration paths like `.env` references might momentarily falter during the transition.
> 
> Required for crates.io: You will need to provide me with a target **License** (e.g., `MIT`, or `MIT OR Apache-2.0`) to append to the metadata.

## Proposed Changes

---

### Root Workspace
Update the centralized configuration to map the newly identified directories natively.

#### [MODIFY] [Cargo.toml](file:///aivolution/projects/chaiss/Cargo.toml)
- Update `members = ["core", "desktop"]` to `members = ["chaiss-core", "chaiss"]`.

---

### chaiss-core
The foundational AI engine and geometrical logic handler.

#### [MODIFY] [core/Cargo.toml](file:///aivolution/projects/chaiss/core/Cargo.toml)
- Rename `name` field from `"chaiss_core"` to `"chaiss-core"`.
- Inject `crates.io` strict metadata (`description`, `license`, `repository`, `keywords` like `["chess", "llm", "ai"]`, `categories`).

#### Directory Physical Translation
- The entire `core/` folder will forcibly be translated to `chaiss-core/` using the native bash `mv` command.

---

### chaiss
The fundamental Egui desktop application providing the GUI and overarching SQLite integrations natively.

#### [MODIFY] [desktop/Cargo.toml](file:///aivolution/projects/chaiss/desktop/Cargo.toml)
- Rename `name` field from `"desktop"` to `"chaiss"`.
- Update the internal dependency array to bind `chaiss-core = { version = "0.1.0", path = "../chaiss-core" }` strictly!
- Inject required `crates.io` metadata (`description`, `license`, `repository`, `keywords` like `["egui", "chess", "desktop", "llm"]`).

#### Directory Physical Translation
- The entire `desktop/` folder will forcefully translate to `chaiss/` natively.

## Open Questions

> [!IMPORTANT]
> 1. Which License do you plan on publishing under? (`MIT`, `Apache-2.0`, `GPL-3.0`, etc.)? Crates.io physically requires a valid SPDX license identifier.
> 2. What `repository` URL should I append dynamically for the metadata (e.g., `https://github.com/a1v0lut10n/chaiss`)?

## Verification Plan

### Automated Tests
- Run `cargo check --workspace` to ensure cross-crate bindings successfully resolve the newly named modules.
- Run `cargo test` implicitly.

### Manual Verification
- We will dynamically dry-run `cargo publish --dry-run -p chaiss-core` to mathematically guarantee crates.io validates all the metadata safely!
