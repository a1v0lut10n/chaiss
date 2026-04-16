# Cross-Platform SQLite Database Initialization

The user raised a valid concern: `tools/scripts/init_db.sh` uses `sqlite3` natively which is excellent on Linux/macOS but breaks portability for Windows developers. We should make the database creation entirely OS-agnostic natively in Rust.

Instead of hacking a `build.rs` to do this (which would painfully slow down compile times by pulling database crates into the build script phase), the standard approach in the Rust ecosystem is to use **`sqlx` Migrations**.

## Proposed Changes

### 1. Migrations Directory
We will extract the SQL data modeling from the bash script into standard `sqlx` migration files.
- **[DELETE]** `tools/scripts/init_db.sh`
- **[NEW]** `migrations/20260415000000_init.sql` (Will contain the `CREATE TABLE` definitions).

### 2. Application Startup (`core/src/db.rs`)
Currently, Chaiss will panic if `chaiss.db` is missing on startup. We will upgrade the connection parameters to explicitly generate the `.db` artifact if it does not physically exist. 

**[MODIFY]** `core/src/db.rs`
- Swap `SqlitePoolOptions::new().connect(database_url)` for a declarative configuration using `SqliteConnectOptions`.
- Add `.create_if_missing(true)` to ensure that the physical file is created dynamically without terminal inputs if omitted.
- Run `sqlx::migrate!().run(&pool).await` immediately after connecting so the newly generated file is instantly hydrated with the schema defined in `migrations/`.

### 3. Documentation Adjustments
**[MODIFY]** `README.md`
- Remove the step about running `./tools/scripts/init_db.sh`.
- Re-word to mention that `cargo run` automatically builds and hydrates the SQLite file on the fly natively on any operating system!

## User Review Required

Does this native migration pipeline sound like a good solution? It's standard in the Rust community, removes any bash dependencies, and is 100% portable for Windows users!
