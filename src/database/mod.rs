//! SQLite persistence layer for sessions and memory.

use anyhow::{Context, Result};
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::PathBuf;

/// SQLite-backed database handle.
#[derive(Debug)]
pub struct Database {
    path: PathBuf,
    conn: Mutex<Connection>,
}

impl Database {
    /// Open (or create) the application database in the data directory.
    pub fn new() -> Result<Self> {
        let dir = crate::utils::paths::Paths::data_dir()
            .unwrap_or_else(|| std::env::temp_dir().join("my-code"));
        crate::utils::paths::Paths::ensure_dir(&dir)?;
        Self::open(dir.join("my-code.db"))
    }

    /// Open a database at an explicit path.
    pub fn open(path: PathBuf) -> Result<Self> {
        if let Some(parent) = path.parent() {
            crate::utils::paths::Paths::ensure_dir(parent)?;
        }
        let conn = Connection::open(&path)
            .with_context(|| format!("Failed to open database at {}", path.display()))?;
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS memory (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )?;
        Ok(Self {
            path,
            conn: Mutex::new(conn),
        })
    }

    /// Path to the database file.
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// Run a closure with a borrowed connection.
    pub fn with_conn<T>(&self, f: impl FnOnce(&Connection) -> Result<T>) -> Result<T> {
        let conn = self.conn.lock();
        f(&conn)
    }

    /// Checkpoint WAL and flush. The connection stays open for reuse.
    pub fn close(&self) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
            .optional_ok();
        Ok(())
    }
}

trait OptionalOk {
    fn optional_ok(self);
}

impl OptionalOk for rusqlite::Result<()> {
    fn optional_ok(self) {
        if let Err(err) = self {
            tracing::debug!("database close checkpoint skipped: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn opens_and_initializes_schema() {
        let dir = tempdir().unwrap();
        let db = Database::open(dir.path().join("test.db")).unwrap();
        db.with_conn(|conn| {
            let count: i64 = conn.query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='sessions'",
                [],
                |row| row.get(0),
            )?;
            assert_eq!(count, 1);
            Ok(())
        })
        .unwrap();
        db.close().unwrap();
    }
}
