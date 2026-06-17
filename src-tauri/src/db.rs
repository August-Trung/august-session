use rusqlite::{Connection, Result};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize, Clone, Debug)]
pub struct MomentRecord {
    pub id: String,
    pub words: String,
    pub windows: String,
    pub screenshot: String,
    pub created_at: String,
}

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

pub fn save_moment(
    conn: &Connection,
    id: &str,
    words: &str,
    windows: &str,
    screenshot: &str,
    created_at: &str,
) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO moments (id, words, windows, screenshot, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        [id, words, windows, screenshot, created_at],
    )?;
    Ok(())
}

pub fn get_moments(conn: &Connection) -> Result<Vec<MomentRecord>> {
    let mut stmt = conn.prepare("SELECT id, words, windows, screenshot, created_at FROM moments ORDER BY created_at DESC")?;
    let moment_iter = stmt.query_map([], |row| {
        Ok(MomentRecord {
            id: row.get(0)?,
            words: row.get(1)?,
            windows: row.get(2)?,
            screenshot: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;

    let mut moments = Vec::new();
    for moment in moment_iter {
        moments.push(moment?);
    }
    Ok(moments)
}

pub fn delete_moment(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM moments WHERE id = ?1", [id])?;
    Ok(())
}
