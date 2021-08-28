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
            Ok(_) => (),
            Err(error) => panic!("{}", error),
        }
        match self.connection.execute(
            "CREATE TABLE IF NOT EXISTS guilds (
                guild_id   UNSIGNED BIG INT PRIMARY KEY,
                enabled    INTEGER DEFAULT FALSE
            )",
            [],
        ) {
            Ok(_) => (),
            Err(error) => panic!("{}", error),
        }
        match self.connection.execute(
            "CREATE TABLE IF NOT EXISTS allowlist (
                id         INTEGER PRIMARY KEY,
                guild_id   UNSIGNED BIG INT NOT NULL, -- Decided to not make this a reference to the guild table for now
                user_id    UNSIGNED BIG INT NOT NULL,
                UNIQUE(guild_id, user_id)
            )",
            [],
        ) {
            Ok(_) => (),
            Err(error) => panic!("{}", error),
        }
        match self.connection.execute(
            "CREATE TABLE IF NOT EXISTS forbidden_words (
                id    INTEGER PRIMARY KEY,
                word  TEXT NOT NULL UNIQUE
            )",
            [],
        ) {
            Ok(_) => (),
            Err(error) => panic!("{}", error),
        }
        println!("Successfully initiated schema");
        let _ = self.insert_in_forbidden_words("://diskord.".to_string());
        let _ = self.insert_in_forbidden_words("://discorcl.".to_string());
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

    pub fn insert_in_allowlist(&self, guild_id: u64, user_id: u64) -> Result<bool> {
        return match self.connection.execute("INSERT INTO allowlist (guild_id, user_id) VALUES (?1, ?2)", params![guild_id, user_id]) {
            Ok(_) => Ok(true),
            Err(error) => Err(error),
        };
    }

    pub fn remove_from_allowlist(&self, guild_id: u64, user_id: u64) -> Result<bool> {
        return match self.connection.execute("DELETE FROM allowlist WHERE guild_id = ?1 AND user_id = ?2", params![guild_id, user_id]) {
            Ok(_) => Ok(true),
            Err(error) => Err(error),
        };
    }

    pub fn is_allowlisted(&self, guild_id: u64, user_id: u64) -> Result<bool> {
        let mut statement = self.connection.prepare("SELECT user_id FROM allowlist WHERE guild_id = ?1 AND user_id = ?2 LIMIT 1")?;
        let mut rows = statement.query([guild_id, user_id])?;
        while let Some(_row) = rows.next()? {
            return Ok(true);
        }
        return Ok(false);
    }

    pub fn count_allowlisted_users_in_guild(&self, guild_id: u64) -> Result<u64> {
        let mut statement = self.connection.prepare("SELECT COUNT(1) FROM allowlist WHERE guild_id = ?1")?;
        let mut rows = statement.query([guild_id])?;
        let mut count: u64 = 0;
        while let Some(row) = rows.next()? {
            count = row.get(0).unwrap();
        }
        return Ok(count);
    }

    pub fn get_allowlisted_users_in_guild(&self, guild_id: u64) -> Result<Vec<u64>> {
        let mut statement = self.connection.prepare("SELECT user_id FROM allowlist WHERE guild_id = ?1")?;
        let mut rows = statement.query([guild_id])?;
        let mut user_ids: Vec<u64> = Vec::new();
        while let Some(row) = rows.next()? {
            user_ids.push(row.get(0).unwrap());
        }
        return Ok(user_ids);
    }

    pub fn insert_in_forbidden_words(&self, word: String) -> Result<bool> {
        return match self.connection.execute("INSERT INTO forbidden_words (word) VALUES (?1)", params![word]) {
            Ok(_) => Ok(true),
            Err(error) => Err(error),
        };
    }

    pub fn remove_from_forbidden_words(&self, word: String) -> Result<bool> {
        return match self.connection.execute("DELETE FROM forbidden_words WHERE word = ?1", params![word]) {
            Ok(_) => Ok(true),
            Err(error) => Err(error),
        };
    }

    pub fn contains_forbidden_word(&self, message: String) -> Result<bool> {
        let mut statement = self.connection.prepare("SELECT word FROM forbidden_words WHERE ?1 LIKE '%'||word||'%' LIMIT 1")?;
        let mut rows = statement.query([message])?;
        while let Some(_row) = rows.next()? {
            return Ok(true);
        }
        return Ok(false);
    }

    pub fn get_forbidden_words(&self) -> Result<Vec<String>> {
        let mut statement = self.connection.prepare("SELECT word FROM forbidden_words")?;
        let mut rows = statement.query([])?;
        let mut forbidden_words: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            forbidden_words.push(row.get(0).unwrap());
        }
        return Ok(forbidden_words);
    }
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}
