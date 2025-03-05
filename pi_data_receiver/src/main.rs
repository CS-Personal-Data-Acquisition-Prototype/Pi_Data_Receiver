use std::error::Error;
use std::thread;
use std::time::Duration;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use crate::connection::wifi::WifiConnection;

mod config;
mod connection;
mod database;
mod models;
mod platform;

fn main() -> Result<(), Box<dyn Error>> {
    let platform_name = "pi_zero";
    let config = config::Config::load(platform_name)?;
    let mut connection_manager = connection::manager::ConnectionManager::new();
    let conn = database::get_db_connection(&config)?;
    
    println!("Starting data receiver...");
    println!("USB Device Path: {}", config.default.usb_device_path);
    println!("WiFi SSID: {}", config.default.wifi_ssid);

    // Start WiFi server
    let address = "0.0.0.0:7878";
    println!("Starting WiFi server on {}...", address);
    
    match WifiConnection::new(address) {
        Ok(wifi_connection) => {
            println!("WiFi server started. Waiting for connections...");
            
            // Spawn a thread to handle connections
            thread::spawn(move || {
                loop {
                    match wifi_connection.accept_connection() {
                        Ok(mut stream) => {
                            println!("New client connected!");
                            
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
                                            // Process data here
                                            
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