use rusqlite::{Connection};
pub use rusqlite::Result;
use log::info;


pub fn init_db() -> Result<Connection> {
    std::fs::create_dir_all("/botLarry/data").map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let conn = Connection::open("/botLarry/data/calls.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            areacode TEXT NOT NULL,
            phonenumber TEXT NOT NULL,
            recording_path TEXT NOT NULL,
            note TEXT DEFAULT '',
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(conn)
}

pub fn show_call_logs(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, areacode, phonenumber, recording_path, note, timestamp FROM calls ORDER BY timestamp DESC"
    )?;

    let call_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    info!("\n 📄 Call Log:");
    for call in call_iter {
        let (id, areacode, number, recording, note, timestamp) = call?;
        info!(
            "[{}] ({}) {} => {} [{}] at {}",
            id, areacode, number, recording, note, timestamp
        );
    }

    Ok(())
}

