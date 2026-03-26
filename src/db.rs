use std::time::{self, SystemTime};
use iroh::endpoint::ConnectionError;
use rusqlite::{params, Connection, Result, OptionalExtension};

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
    let now = SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    conn.execute("UPDATE notes SET data = ?2, time = ?3 WHERE name = ?1", params![name, data, now])?;
    Ok(())
}

pub fn read_note(conn: &Connection, name: String) -> Result<Option<String>> {
    conn.query_row(
        "SELECT data FROM notes WHERE name = ?1",
        params![name],
        |row| row.get(0),
    ).optional()
}

pub fn get_time(conn: &Connection, name: String) -> Result<Option<i64>>{
    conn.query_row(
        "SELECT time FROM notes WHERE name = ?1",
        params![name],
        |row| row.get(0),
    ).optional()
}

pub fn read_note_names(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM notes")?;
    let names = stmt.query_map([], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<String>>>()?;
    Ok(names)
}