use std::error::Error;
use std::thread;
use std::time::Duration;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use crate::connection::wifi::WifiConnection;
use rusqlite::Connection;

mod config;
mod connection;
mod database;
mod models;
mod platform;

fn process_sensor_data(conn: &Connection, data_str: &str) -> Result<(), Box<dyn Error>> {
    println!("Processing data: {}", data_str);
    
    // Initialize variables
    let mut session_id: Option<i32> = None;
    let mut timestamp = String::new();
    let mut latitude = 0.0;
    let mut longitude = 0.0;
    let mut altitude = 0.0;
    let mut accel_x = 0.0;
    let mut accel_y = 0.0;
    let mut accel_z = 0.0;
    let mut gyro_x = 0.0;
    let mut gyro_y = 0.0;
    let mut gyro_z = 0.0;
    let mut dac_1 = 0.0;
    let mut dac_2 = 0.0;
    let mut dac_3 = 0.0;
    let mut dac_4 = 0.0;

    // TODO DEBUG 
    println!("Data length: {}, parts: {}", data_str.len(), data_str.split(',').count());
    
    // Parse CSV format
    let parts: Vec<&str> = data_str.split(',').collect();
    if parts.len() >= 15 {
        // Skip the first value if it's an ID or row number
        let offset = if parts.len() > 15 { 1 } else { 0 };
        
        if let Ok(sid) = parts[offset].parse::<i32>() {
            session_id = Some(sid);
        }
        timestamp = parts[offset + 1].to_string();
        latitude = parts[offset + 2].parse().unwrap_or(0.0);
        longitude = parts[offset + 3].parse().unwrap_or(0.0);
        altitude = parts[offset + 4].parse().unwrap_or(0.0);
        accel_x = parts[offset + 5].parse().unwrap_or(0.0);
        accel_y = parts[offset + 6].parse().unwrap_or(0.0);
        accel_z = parts[offset + 7].parse().unwrap_or(0.0);
        gyro_x = parts[offset + 8].parse().unwrap_or(0.0);
        gyro_y = parts[offset + 9].parse().unwrap_or(0.0);
        gyro_z = parts[offset + 10].parse().unwrap_or(0.0);
        dac_1 = parts[offset + 11].parse().unwrap_or(0.0);
        dac_2 = parts[offset + 12].parse().unwrap_or(0.0);
        dac_3 = parts[offset + 13].parse().unwrap_or(0.0);
        dac_4 = parts[offset + 14].parse().unwrap_or(0.0);
        
        // TODO DEBUG
        println!("Parsed values: sid={:?}, ts={}, lat={}, long={}, alt={}...", 
            session_id, timestamp, latitude, longitude, altitude);
    } else {
        println!("Warning: Invalid data format. Expected CSV with 15+ fields, got: {}", data_str);
    }
    
    // Insert into database
    println!("Attempting database insert...");
    let result = database::insert_sensor_data(
        conn, session_id, &timestamp, latitude, longitude, altitude,
        accel_x, accel_y, accel_z, gyro_x, gyro_y, gyro_z,
        dac_1, dac_2, dac_3, dac_4
    );
    
    match result {
        Ok(_) => println!("Database insert successful"),
        Err(e) => println!("Database insert failed: {}", e),
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let platform_name = "pi_zero";
    let config = config::Config::load(platform_name)?;
    let mut connection_manager = connection::manager::ConnectionManager::new();
    
    // Wrap database connection in Arc<Mutex<>> to share across threads
    let conn = Arc::new(Mutex::new(database::get_db_connection(&config)?));
    
    println!("Starting data receiver...");
    println!("USB Device Path: {}", config.default.usb_device_path);
    println!("WiFi SSID: {}", config.default.wifi_ssid);

    // Start WiFi server
    let address = "0.0.0.0:7878";
    println!("Starting WiFi server on {}...", address);
    
    // Clone the connection for the WiFi thread
    let thread_conn = Arc::clone(&conn);
    
    match WifiConnection::new(address) {
        Ok(wifi_connection) => {
            println!("WiFi server started. Waiting for connections...");
            
            // Spawn a thread to handle connections
            thread::spawn(move || {
                loop {
                    match wifi_connection.accept_connection() {
                        Ok(mut stream) => {
                            println!("New client connected!");
                            
                            // Clone connection for this client thread
                            let client_conn = Arc::clone(&thread_conn);
                            
                            // Handle each client in a new thread
                            let _ = thread::spawn(move || {
                                loop {
                                    match stream.set_read_timeout(Some(Duration::from_secs(1))) {
                                        Ok(_) => {},
                                        Err(e) => {
                                            eprintln!("Error setting read timeout: {}", e);
                                            break;
                                        }
                                    }
                                    
                                    let mut buffer = [0; 1024];
                                    match stream.read(&mut buffer) {
                                        Ok(0) => break, // Connection closed
                                        Ok(n) => {
                                            let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                                            println!("Received: {}", data);
                                            
                                            // Process and save data
                                            if let Ok(db_conn) = client_conn.lock() {
                                                if let Err(e) = process_sensor_data(&db_conn, &data) {
                                                    eprintln!("Error processing data: {}", e);
                                                }
                                            }
                                            
                                            // Example response
                                            if let Err(e) = stream.write_all(b"ACK\n") {
                                                eprintln!("Error sending response: {}", e);
                                                break;
                                            }
                                        },
                                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                            // Timeout, continue
                                            continue;
                                        },
                                        Err(e) => {
                                            eprintln!("Error reading from client: {}", e);
                                            break;
                                        }
                                    }
                                }
                                println!("Client disconnected");
                            });
                        },
                        Err(e) => eprintln!("Connection error: {}", e),
                    }
                }
            });
        },
        Err(e) => eprintln!("Failed to start WiFi server: {}", e),
    }

    // Main loop
    loop {
        connection_manager.check_connections()?;
        thread::sleep(Duration::from_millis(100));
    }
}