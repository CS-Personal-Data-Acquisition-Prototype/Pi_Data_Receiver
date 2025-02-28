use rusqlite::{params, Connection};
use std::error::Error;

pub fn init_schema(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensor_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sessionID INTEGER,
            timestamp TEXT,
            latitude REAL,
            longitude REAL,
            altitude REAL,
            accel_x REAL,
            accel_y REAL,
            accel_z REAL,
            gyro_x REAL,
            gyro_y REAL,
            gyro_z REAL,
            dac_1 REAL,
            dac_2 REAL,
            dac_3 REAL,
            dac_4 REAL
        )",
        [],
    )?;
    Ok(())
}