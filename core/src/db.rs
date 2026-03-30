// SQLite sqlx persistence logic
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

    pub async fn get_or_create_player(&self, name: &str) -> Result<i64, Error> {
        if let Some(id) = self.get_player_by_name(name).await? {
            return Ok(id);
        }
        self.create_player(name).await
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

    pub async fn create_game(&self, name: &str, white_id: i64, black_id: i64, initial_fen: &str) -> Result<i64, Error> {
        let status = "ongoing";
        let result = sqlx::query!(
            "INSERT INTO games (name, white_player_id, black_player_id, current_fen, status) VALUES (?, ?, ?, ?, ?)",
            name, white_id, black_id, initial_fen, status
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
        
        Ok(records.into_iter().map(|rec| GameRecord {
            id: rec.id,
            name: rec.name,
            status: rec.status,
            white_player: rec.white_name,
            black_player: rec.black_name,
        }).collect())
    }

    /// Recursively fetches the exact historical move vectors for resuming Egui Sandbox arrays natively!
    pub async fn load_game_history(&self, game_id: i64) -> Result<(String, Vec<String>), sqlx::Error> {
        // Technically, `games` does not retain `initial_fen` independently currently. We inject standard FIDE root explicitly!
        let root_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

        let moves = sqlx::query!(
            "SELECT fen_snapshot FROM moves WHERE game_id = ? ORDER BY move_number ASC",
            game_id
        )
        .fetch_all(&self.pool)
        .await?;

        let history = moves.into_iter().map(|r| r.fen_snapshot).collect();
        Ok((root_fen, history))
    }
}
