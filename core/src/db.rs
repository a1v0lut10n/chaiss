// SQLite sqlx persistence logic
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Error;

pub struct DbClient {
    pool: SqlitePool,
}

impl DbClient {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn create_player(&self, name: &str) -> Result<i64, Error> {
        let result = sqlx::query!(
            "INSERT INTO players (name) VALUES (?)",
            name
        )
        .execute(&self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_player_by_name(&self, name: &str) -> Result<Option<i64>, Error> {
        let record = sqlx::query!(
            "SELECT id FROM players WHERE name = ?",
            name
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(record.and_then(|r| r.id)) // flatten the implicitly wrapped SQLite Option
    }

    pub async fn create_game(&self, white_id: i64, black_id: i64, initial_fen: &str) -> Result<i64, Error> {
        let status = "ongoing";
        let result = sqlx::query!(
            "INSERT INTO games (white_player_id, black_player_id, current_fen, status) VALUES (?, ?, ?, ?)",
            white_id, black_id, initial_fen, status
        )
        .execute(&self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn update_game_state(&self, game_id: i64, current_fen: &str, status: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE games SET current_fen = ?, status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            current_fen, status, game_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn log_move(&self, game_id: i64, move_number: i64, fen_snapshot: &str, notation: &str) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO moves (game_id, move_number, fen_snapshot, notation) VALUES (?, ?, ?, ?)",
            game_id, move_number, fen_snapshot, notation
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
}
