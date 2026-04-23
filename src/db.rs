use std::time::{self, SystemTime};
use iroh::{self, Endpoint, EndpointAddr};
use rusqlite::{params, Connection, Result, OptionalExtension};

pub fn open_db() -> Result<Connection> {
    let conn = Connection::open("notes.db")?;
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            data TEXT,
            time INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            name TEXT NOT NULL,
            node_id TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS connected_users (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            node_id TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS user_notes (
            user_id  INTEGER NOT NULL REFERENCES connected_users(id),
            note_id  INTEGER NOT NULL REFERENCES notes(id),
            PRIMARY KEY (user_id, note_id)
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
        "INSERT OR IGNORE INTO notes (name, data, time) VALUES (?1, ?2, ?3)",
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

pub fn rename_note(conn: &Connection, name: String, new: String) -> Result<bool> {
    let affected = conn.execute(
        "UPDATE notes SET name = ?1 WHERE name = ?2",
        params![new, name],
    )?;
    Ok(affected > 0)
}

// ===== USER =====

pub fn create_user(conn: &Connection, name: String, node_id: String) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO user (id, name, node_id) VALUES (1, ?1, ?2)",
        params![name, node_id],
    )?;
    Ok(())
}

pub fn get_user(conn: &Connection) -> Result<Option<(String, String)>> {
    conn.query_row(
        "SELECT name, node_id FROM user WHERE id = 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).optional()
}

pub fn update_user(conn: &Connection, name: String, node_id: String) -> Result<bool> {
    let affected = conn.execute(
        "UPDATE user SET name = ?1, node_id = ?2 WHERE id = 1",
        params![name, node_id],
    )?;
    Ok(affected > 0)
}

// ===== CONNECTED USERS =====

pub fn create_connected_user(conn: &Connection, name: String, node_id: String) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO connected_users (name, node_id) VALUES (?1, ?2)",
        params![name, node_id],
    )?;
    Ok(())
}

pub fn get_connected_user_by_id(conn: &Connection, id: i64) -> Result<Option<(String, String)>> {
    conn.query_row(
        "SELECT name, node_id FROM connected_users WHERE id = ?1",
        params![id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).optional()
}

pub fn get_connected_user_by_node_id(conn: &Connection, node_id: String) -> Result<Option<(i64, String)>> {
    conn.query_row(
        "SELECT id, name FROM connected_users WHERE node_id = ?1",
        params![node_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).optional()
}

pub fn get_all_connected_users(conn: &Connection) -> Result<Vec<(i64, String, String)>> {
    let mut stmt = conn.prepare("SELECT id, name, node_id FROM connected_users")?;
    let users = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .collect::<rusqlite::Result<Vec<(i64, String, String)>>>()?;
    Ok(users)
}

pub fn delete_connected_user(conn: &Connection, node_id: String) -> Result<bool> {
    let affected = conn.execute(
        "DELETE FROM connected_users WHERE node_id = ?1",
        params![node_id],
    )?;
    Ok(affected > 0)
}

// ===== USER NOTES =====

pub fn link_user_note(conn: &Connection, user_id: i64, note_id: i64) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO user_notes (user_id, note_id) VALUES (?1, ?2)",
        params![user_id, note_id],
    )?;
    Ok(())
}

pub fn unlink_user_note(conn: &Connection, user_id: i64, note_id: i64) -> Result<bool> {
    let affected = conn.execute(
        "DELETE FROM user_notes WHERE user_id = ?1 AND note_id = ?2",
        params![user_id, note_id],
    )?;
    Ok(affected > 0)
}

pub fn get_note_ids_by_user(conn: &Connection, user_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare(
        "SELECT note_id FROM user_notes WHERE user_id = ?1",
    )?;
    let ids = stmt.query_map(params![user_id], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<i64>>>()?;
    Ok(ids)
}

pub fn get_user_ids_by_note(conn: &Connection, note_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare(
        "SELECT user_id FROM user_notes WHERE note_id = ?1",
    )?;
    let ids = stmt.query_map(params![note_id], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<i64>>>()?;
    Ok(ids)
}

pub fn unlink_all_notes_from_user(conn: &Connection, user_id: i64) -> Result<usize> {
    let affected = conn.execute(
        "DELETE FROM user_notes WHERE user_id = ?1",
        params![user_id],
    )?;
    Ok(affected)
}