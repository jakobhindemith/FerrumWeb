use reqwest::Error;
use rusqlite::{params, Connection, Result};

fn init_db() -> Result<Connection> {
    let conn = Connection::open("links.db")?;
    //create Table with all informations
    conn.execute(
        "CREATE TABLE IF NOT EXIST link (
        id INTEGER PRIMARY KEY;
        URL TEXT NOT NULL
        parent_id INTEGER
        depth INTEGER
        )",
        (),
    )?;
    Ok((conn))
}

pub fn insert_link(conn: &Connection, url: String, depth: usize) -> Result<i64>{
    conn.execute("INSERT INTO link (url, depth) VALUES (?1, ?2)",
    params![url, depth as i64],
    )?;
Ok((conn.last_insert_rowid()))
}