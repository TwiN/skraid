use rusqlite::{params, Connection, Result};
use serenity::prelude::TypeMapKey;

pub struct Database {
    connection: Connection,
}

impl TypeMapKey for Database {
    type Value = Database;
}

impl Database {
    pub fn new(path: String) -> Database {
        let database: Database;
        if path.is_empty() {
            database = Database {
                connection: Connection::open_in_memory().expect("connection to database should've been established"),
            };
        } else {
            database = Database {
                connection: Connection::open(path).expect("connection to database should've been established"),
            };
        }
        database.create_schema();
        database
    }

    pub fn create_schema(&self) {
        match self.connection.execute(
            "CREATE TABLE IF NOT EXISTS blocklist (
                user_id   UNSIGNED BIG INT PRIMARY KEY,
                reason    TEXT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ) {
            Ok(_) => println!("Successfully initiated schema"),
            Err(error) => panic!("{}", error),
        }
    }

    pub fn insert_in_blocklist(&self, id: u64, reason: String) -> Result<bool> {
        return match self.connection.execute("INSERT INTO blocklist (user_id, reason) VALUES (?1, ?2)", params![id, reason]) {
            Ok(_) => Ok(true),
            Err(error) => Err(error),
        };
    }

    pub fn remove_from_blocklist(&self, id: u64) -> Result<bool> {
        return match self.connection.execute("DELETE FROM blocklist WHERE user_id = ?1", params![id]) {
            Ok(_) => Ok(true),
            Err(error) => Err(error),
        };
    }

    pub fn is_blocklisted(&self, id: u64) -> Result<bool> {
        let mut statement = self.connection.prepare("SELECT user_id FROM blocklist WHERE user_id = ? LIMIT 1")?;
        let mut rows = statement.query([id])?;
        while let Some(_row) = rows.next()? {
            return Ok(true);
        }
        return Ok(false);
    }
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}
