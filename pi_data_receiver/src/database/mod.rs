use rusqlite::{params, Connection};
use std::error::Error;

pub fn init_db(conn: &Connection) -> Result<(), Box<dyn Error>> {
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

pub fn insert_sensor_data(conn: &Connection, session_id: Option<i32>, timestamp: &str, latitude: f64, longitude: f64, altitude: f64, accel_x: f64, accel_y: f64, accel_z: f64, gyro_x: f64, gyro_y: f64, gyro_z: f64, dac_1: f64, dac_2: f64, dac_3: f64, dac_4: f64) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "INSERT INTO sensor_data (
            sessionID, timestamp, latitude, longitude, altitude,
            accel_x, accel_y, accel_z,
            gyro_x, gyro_y, gyro_z,
            dac_1, dac_2, dac_3, dac_4
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![
            session_id,
            timestamp,
            latitude,
            longitude,
            altitude,
            accel_x,
            accel_y,
            accel_z,
            gyro_x,
            gyro_y,
            gyro_z,
            dac_1,
            dac_2,
            dac_3,
            dac_4,
        ],
    )?;
    Ok(())
}

pub fn query_sensor_data(conn: &Connection) -> Result<Vec<(Option<i32>, String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT sessionID, timestamp, latitude, longitude, altitude, 
                                      accel_x, accel_y, accel_z, 
                                      gyro_x, gyro_y, gyro_z, 
                                      dac_1, dac_2, dac_3, dac_4 FROM sensor_data")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            row.get(9)?,
            row.get(10)?,
            row.get(11)?,
            row.get(12)?,
            row.get(13)?,
            row.get(14)?,
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}