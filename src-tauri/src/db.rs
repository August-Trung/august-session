use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

pub fn init_db(app_data_dir: &Path) -> Result<Connection> {
    // Ensure parent directory exists
    if !app_data_dir.exists() {
        fs::create_dir_all(app_data_dir).expect("Failed to create app data directory");
    }

    let db_path = app_data_dir.join("august.db");
    let conn = Connection::open(&db_path)?;

    // Create moments table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS moments (
            id          TEXT PRIMARY KEY,
            words       TEXT,
            windows     TEXT NOT NULL,  -- JSON string
            screenshot  TEXT NOT NULL,  -- Relative filename in screenshots dir
            created_at  TEXT NOT NULL   -- ISO 8601 string
        );",
        [],
    )?;

    // Create index on created_at DESC
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_moments_created_at ON moments(created_at DESC);",
        [],
    )?;

    Ok(conn)
}
