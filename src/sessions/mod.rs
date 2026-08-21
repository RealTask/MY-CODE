//! Session management and persistence.

use anyhow::Result;
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use uuid::Uuid;

use crate::database::Database;

/// A conversation / work session.
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    /// Create a new in-memory session.
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.into(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Creates, lists, and stores sessions.
#[derive(Debug)]
pub struct SessionManager {
    sessions: Mutex<Vec<Session>>,
}

impl SessionManager {
    /// Create a session manager. The database is used to persist sessions when
    /// available; failures to persist are non-fatal for in-memory use.
    pub fn new(database: &Database) -> Result<Self> {
        let sessions = load_sessions(database).unwrap_or_default();
        Ok(Self {
            sessions: Mutex::new(sessions),
        })
    }

    /// Create and store a new session.
    pub fn create(&self, title: impl Into<String>) -> Session {
        let session = Session::new(title);
        self.sessions.lock().push(session.clone());
        session
    }

    /// List known sessions.
    pub fn list(&self) -> Vec<Session> {
        self.sessions.lock().clone()
    }

    /// Look up a session by id.
    pub fn get(&self, id: &str) -> Option<Session> {
        self.sessions.lock().iter().find(|s| s.id == id).cloned()
    }

    /// Delete a session by id. Returns whether a session was removed.
    pub fn delete(&self, id: &str) -> bool {
        let mut sessions = self.sessions.lock();
        let len = sessions.len();
        sessions.retain(|s| s.id != id);
        sessions.len() != len
    }

    /// Remove all sessions.
    pub fn clear(&self) {
        self.sessions.lock().clear();
    }
}

fn load_sessions(database: &Database) -> Result<Vec<Session>> {
    database.with_conn(|conn| {
        let mut stmt =
            conn.prepare("SELECT id, title, created_at, updated_at FROM sessions ORDER BY created_at")?;
        let rows = stmt.query_map([], |row| {
            let created_at = parse_time(row.get::<_, String>(2)?);
            let updated_at = parse_time(row.get::<_, String>(3)?);
            Ok(Session {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at,
                updated_at,
            })
        })?;
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(row?);
        }
        Ok(sessions)
    })
}

fn parse_time(value: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_and_get_session() {
        let dir = tempdir().unwrap();
        let db = Database::open(dir.path().join("s.db")).unwrap();
        let manager = SessionManager::new(&db).unwrap();
        let session = manager.create("test");
        assert_eq!(manager.get(&session.id).unwrap().title, "test");
        assert_eq!(manager.list().len(), 1);
        assert!(manager.delete(&session.id));
        assert!(manager.list().is_empty());
    }
}
