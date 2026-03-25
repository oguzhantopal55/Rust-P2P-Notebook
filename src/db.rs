use std::time::{self, SystemTime};
use iroh::endpoint::ConnectionError;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Note {
    pub id: i32,
    pub name: String,
    pub data: Option<String>,
    pub time: i64,
}

pub fn open_db() -> Result<Connection> {
    let conn = Connection::open("notes.db")?;
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data TEXT,
            time INTEGER NOT NULL
        );
    ")?;
    Ok(conn)
}

pub fn create_note(conn: &Connection, name: String) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    conn.execute(
        "INSERT INTO notes (name, data, time) VALUES (?1, ?2, ?3)",
        params![name, Some("KRALIMIZ COK YASA".to_string()), now],
    )?;
    Ok(())
}

pub fn write_note(conn: &Connection, data: String, name: String) -> Result<()> {
    conn.execute("UPDATE notes SET data = ?2 WHERE name = ?1", params![name, data])?;
    Ok(())
}