// SQLite sqlx persistence logic
use sqlx::migrate::MigrateError;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Error;

#[derive(Debug, Clone)]
pub struct GameRecord {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub white_player: String,
    pub black_player: String,
}

pub struct DbClient {
    pool: SqlitePool,
}

/// Explains a failed [`DbClient::new`] to the person launching the app: which
/// file, what went wrong, and what to do about it. Migration-ledger failures
/// get specific guidance, since they mean the file was last touched by a
/// different chaiss build rather than that anything is corrupt.
pub fn describe_open_failure(database_url: &str, err: &Error) -> String {
    let mut text = format!("Could not open the chaiss database `{database_url}`: {err}");
    if let Error::Migrate(migrate_err) = err {
        match **migrate_err {
            MigrateError::VersionMismatch(version) => text.push_str(&format!(
                "\nMigration {version} was applied to this database by a chaiss build whose \
                 copy of that migration differs from this one. Open the file with the chaiss \
                 version that last used it, or back it up and start from a fresh database."
            )),
            MigrateError::VersionMissing(version) => text.push_str(&format!(
                "\nThis database was migrated by a newer chaiss than this one (migration \
                 {version} is unknown here). Upgrade chaiss (`cargo install chaiss`)."
            )),
            MigrateError::Dirty(version) => text.push_str(&format!(
                "\nMigration {version} did not finish on a previous launch. Restore the file \
                 from a backup, or move it aside to start from a fresh database."
            )),
            _ => {}
        }
    }
    text.push_str("\nSet DATABASE_URL to open a different file.");
    text
}

impl DbClient {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        use sqlx::sqlite::SqliteConnectOptions;
        use std::str::FromStr;

        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        // Migrations run on every launch. A database that a *newer* chaiss
        // already migrated carries ledger rows this build does not know about;
        // ignoring them lets an older build keep opening the file, which is
        // safe because migrations are additive-only (ADR-0001). A migration
        // this build does know, but whose checksum differs from the applied
        // one, is still rejected.
        let mut migrator = sqlx::migrate!("./migrations");
        migrator.set_ignore_missing(true);
        migrator.run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn create_player(&self, name: &str) -> Result<i64, Error> {
        let result = sqlx::query!("INSERT INTO players (name) VALUES (?)", name)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_or_create_player(&self, name: &str) -> Result<i64, Error> {
        if let Some(id) = self.get_player_by_name(name).await? {
            return Ok(id);
        }
        self.create_player(name).await
    }

    pub async fn get_player_by_name(&self, name: &str) -> Result<Option<i64>, Error> {
        let record = sqlx::query!("SELECT id FROM players WHERE name = ?", name)
            .fetch_optional(&self.pool)
            .await?;
        Ok(record.and_then(|r| r.id)) // flatten the implicitly wrapped SQLite Option
    }

    pub async fn create_game(
        &self,
        name: &str,
        white_id: i64,
        black_id: i64,
        initial_fen: &str,
    ) -> Result<i64, Error> {
        let status = "ongoing";
        let result = sqlx::query!(
            "INSERT INTO games (name, white_player_id, black_player_id, current_fen, status) VALUES (?, ?, ?, ?, ?)",
            name, white_id, black_id, initial_fen, status
        )
        .execute(&self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn update_game_state(
        &self,
        game_id: i64,
        current_fen: &str,
        status: &str,
    ) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE games SET current_fen = ?, status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            current_fen, status, game_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Persists the board-flip orientation so resuming a session restores the same viewpoint.
    pub async fn set_flip_board(&self, game_id: i64, flipped: bool) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE games SET flip_board = ? WHERE id = ?",
            flipped,
            game_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_flip_board(&self, game_id: i64) -> Result<bool, Error> {
        let record = sqlx::query!("SELECT flip_board FROM games WHERE id = ?", game_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(record.is_some_and(|r| r.flip_board != 0))
    }

    /// Persists the `provider/model` that served the game's most recent LLM
    /// request, so resuming the game restores that selection.
    pub async fn set_last_llm_target(&self, game_id: i64, target: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE games SET last_llm_target = ? WHERE id = ?",
            target,
            game_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_last_llm_target(&self, game_id: i64) -> Result<Option<String>, Error> {
        let record = sqlx::query!("SELECT last_llm_target FROM games WHERE id = ?", game_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(record.and_then(|r| r.last_llm_target))
    }

    pub async fn delete_game(&self, game_id: i64) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        // Cascading Relational teardown structure cleanly wiping dependencies beforehand!
        sqlx::query!("DELETE FROM chat_messages WHERE game_id = ?", game_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM moves WHERE game_id = ?", game_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM games WHERE id = ?", game_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn log_move(
        &self,
        game_id: i64,
        move_number: i64,
        fen_snapshot: &str,
        notation: &str,
    ) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO moves (game_id, move_number, fen_snapshot, notation) VALUES (?, ?, ?, ?)",
            game_id,
            move_number,
            fen_snapshot,
            notation
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Natively handles the "Undo" architecture algebraically without engine reverse-math!
    pub async fn undo_last_move(&self, game_id: i64) -> Result<Option<String>, Error> {
        // Query the highest move_number mathematically tracked
        let last_move = sqlx::query!(
            "SELECT id, move_number FROM moves WHERE game_id = ? ORDER BY move_number DESC LIMIT 1",
            game_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(lm) = last_move {
            // Discard the erroneous action from the physical history ledger natively!
            sqlx::query!("DELETE FROM moves WHERE id = ?", lm.id)
                .execute(&self.pool)
                .await?;

            // Look up what the physical state was directly prior natively by scanning the previous stack vector
            let prev_move = sqlx::query!(
                "SELECT fen_snapshot FROM moves WHERE game_id = ? ORDER BY move_number DESC LIMIT 1",
                game_id
            )
            .fetch_optional(&self.pool)
            .await?;

            let recovered_fen = prev_move.map(|r| r.fen_snapshot).unwrap_or_else(|| {
                // If the player undid absolutely the FIRST move of the game, reset to pristine FIDE starting structures universally!
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
            });

            // Re-mount the recovered array to the active game tracking native structurally!
            sqlx::query!(
                "UPDATE games SET current_fen = ?, status = 'ongoing', updated_at = CURRENT_TIMESTAMP WHERE id = ?",
                recovered_fen, game_id
            )
            .execute(&self.pool)
            .await?;

            return Ok(Some(recovered_fen));
        }

        Ok(None)
    }

    /// Fetches a dynamic matrix of active database sessions mathematically JOINed alongside explicit strings!
    pub async fn get_active_games(&self) -> Result<Vec<GameRecord>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT
                g.id, g.name, g.status,
                pw.name as white_name,
                pb.name as black_name
            FROM games g
            JOIN players pw ON g.white_player_id = pw.id
            JOIN players pb ON g.black_player_id = pb.id
            ORDER BY g.updated_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|rec| GameRecord {
                id: rec.id,
                name: rec.name,
                status: rec.status,
                white_player: rec.white_name,
                black_player: rec.black_name,
            })
            .collect())
    }

    /// Recursively fetches the exact historical move vectors for resuming Egui Sandbox arrays natively!
    pub async fn load_game_history(
        &self,
        game_id: i64,
    ) -> Result<(String, Vec<String>, Vec<String>), sqlx::Error> {
        // Technically, `games` does not retain `initial_fen` independently currently. We inject standard FIDE root explicitly!
        let root_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

        let moves = sqlx::query!(
            "SELECT fen_snapshot, notation FROM moves WHERE game_id = ? ORDER BY move_number ASC",
            game_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut fen_history = Vec::new();
        let mut algebraic_history = Vec::new();

        for r in moves {
            fen_history.push(r.fen_snapshot);
            algebraic_history.push(r.notation);
        }

        Ok((root_fen, fen_history, algebraic_history))
    }

    pub async fn log_chat_message(
        &self,
        game_id: i64,
        role: &str,
        content: &str,
    ) -> Result<i64, Error> {
        let result = sqlx::query!(
            "INSERT INTO chat_messages (game_id, role, content) VALUES (?, ?, ?)",
            game_id,
            role,
            content
        )
        .execute(&self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn load_chat_history(&self, game_id: i64) -> Result<Vec<(String, String)>, Error> {
        let messages = sqlx::query!(
            "SELECT role, content FROM chat_messages WHERE game_id = ? ORDER BY id ASC",
            game_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut chat_history = Vec::new();
        for r in messages {
            chat_history.push((r.role, r.content));
        }

        Ok(chat_history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const ROOT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    /// A SQLite file in a scratch directory of its own, removed on drop.
    struct ScratchDb {
        dir: PathBuf,
        url: String,
    }

    impl ScratchDb {
        fn new() -> Self {
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let dir = std::env::temp_dir().join(format!(
                "chaiss-core-db-test-{}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&dir).expect("create scratch dir");
            let url = format!("sqlite://{}", dir.join("chaiss.db").display());
            Self { dir, url }
        }

        async fn open(&self) -> Result<DbClient, Error> {
            DbClient::new(&self.url).await
        }

        async fn pool(&self) -> SqlitePool {
            use sqlx::sqlite::SqliteConnectOptions;
            use std::str::FromStr;
            let options = SqliteConnectOptions::from_str(&self.url)
                .expect("parse url")
                .create_if_missing(true);
            SqlitePool::connect_with(options).await.expect("connect")
        }
    }

    impl Drop for ScratchDb {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    fn shipped_migrator() -> sqlx::migrate::Migrator {
        sqlx::migrate!("./migrations")
    }

    async fn applied_versions(pool: &SqlitePool) -> Vec<i64> {
        sqlx::query_scalar("SELECT version FROM _sqlx_migrations ORDER BY version")
            .fetch_all(pool)
            .await
            .expect("read migration ledger")
    }

    async fn new_game(db: &DbClient) -> i64 {
        let white = db.get_or_create_player("White").await.expect("white");
        let black = db.get_or_create_player("Black").await.expect("black");
        db.create_game("Test", white, black, ROOT_FEN)
            .await
            .expect("create game")
    }

    #[tokio::test]
    async fn fresh_database_gets_every_shipped_migration() {
        let scratch = ScratchDb::new();
        let db = scratch.open().await.expect("open fresh database");

        let expected: Vec<i64> = shipped_migrator().iter().map(|m| m.version).collect();
        assert_eq!(applied_versions(&db.pool).await, expected);

        let game = new_game(&db).await;
        db.set_last_llm_target(game, "openai/gpt-6-astra")
            .await
            .expect("set target");
        assert_eq!(
            db.get_last_llm_target(game).await.expect("get target"),
            Some("openai/gpt-6-astra".to_string())
        );
    }

    #[tokio::test]
    async fn database_from_the_first_release_is_upgraded_in_place() {
        let scratch = ScratchDb::new();

        // Reproduce a 0.1.x file: only the initial migration applied.
        let init = shipped_migrator()
            .iter()
            .next()
            .expect("initial migration")
            .clone();
        let legacy = sqlx::migrate::Migrator {
            migrations: Cow::Owned(vec![init.clone()]),
            ..sqlx::migrate::Migrator::DEFAULT
        };
        {
            let pool = scratch.pool().await;
            legacy.run(&pool).await.expect("apply initial migration");
            assert_eq!(applied_versions(&pool).await, vec![init.version]);
            pool.close().await;
        }

        let db = scratch.open().await.expect("open legacy database");
        let expected: Vec<i64> = shipped_migrator().iter().map(|m| m.version).collect();
        assert_eq!(applied_versions(&db.pool).await, expected);

        let game = new_game(&db).await;
        assert!(!db.get_flip_board(game).await.expect("flip board"));
        assert_eq!(db.get_last_llm_target(game).await.expect("target"), None);
    }

    #[tokio::test]
    async fn database_migrated_by_a_newer_build_still_opens() {
        let scratch = ScratchDb::new();
        let future_version: i64 = 99_991_231_000_000;
        {
            let db = scratch.open().await.expect("open fresh database");
            sqlx::query(
                "INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time) \
                 VALUES (?, 'from a newer chaiss', TRUE, X'00', 0)",
            )
            .bind(future_version)
            .execute(&db.pool)
            .await
            .expect("insert future ledger row");
            db.pool.close().await;
        }

        let db = scratch
            .open()
            .await
            .expect("an older build must tolerate a newer database");
        assert!(applied_versions(&db.pool).await.contains(&future_version));

        let game = new_game(&db).await;
        db.set_last_llm_target(game, "google/gemini-3.7-flash")
            .await
            .expect("set target");
        assert_eq!(
            db.get_last_llm_target(game).await.expect("get target"),
            Some("google/gemini-3.7-flash".to_string())
        );
    }

    #[tokio::test]
    async fn modified_migration_is_rejected_with_guidance() {
        let scratch = ScratchDb::new();
        let last = shipped_migrator()
            .iter()
            .last()
            .expect("a shipped migration")
            .version;
        {
            let db = scratch.open().await.expect("open fresh database");
            sqlx::query("UPDATE _sqlx_migrations SET checksum = X'00' WHERE version = ?")
                .bind(last)
                .execute(&db.pool)
                .await
                .expect("tamper with ledger checksum");
            db.pool.close().await;
        }

        let err = match scratch.open().await {
            Ok(_) => panic!("a modified migration must be rejected"),
            Err(err) => err,
        };
        assert!(
            matches!(&err, Error::Migrate(m) if matches!(**m, MigrateError::VersionMismatch(v) if v == last)),
            "unexpected error: {err:?}"
        );

        let text = describe_open_failure(&scratch.url, &err);
        assert!(text.contains(&scratch.url), "{text}");
        assert!(text.contains(&last.to_string()), "{text}");
        assert!(text.contains("differs from this one"), "{text}");
        assert!(text.contains("DATABASE_URL"), "{text}");
    }

    #[test]
    fn describe_open_failure_explains_ledger_states() {
        let newer = Error::Migrate(Box::new(MigrateError::VersionMissing(42)));
        let text = describe_open_failure("sqlite://x.db", &newer);
        assert!(text.contains("newer chaiss"), "{text}");
        assert!(text.contains("42"), "{text}");

        let dirty = Error::Migrate(Box::new(MigrateError::Dirty(7)));
        let text = describe_open_failure("sqlite://x.db", &dirty);
        assert!(text.contains("did not finish"), "{text}");

        let other = Error::PoolClosed;
        let text = describe_open_failure("sqlite://x.db", &other);
        assert!(
            text.starts_with("Could not open the chaiss database `sqlite://x.db`"),
            "{text}"
        );
        assert!(text.contains("DATABASE_URL"), "{text}");
    }
}
