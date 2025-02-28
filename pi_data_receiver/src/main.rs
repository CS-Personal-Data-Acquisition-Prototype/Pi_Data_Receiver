use std::error::Error;
use std::thread;
use std::time::Duration;

mod config;
mod connection;
mod database;
mod models;
mod platform;

fn main() -> Result<(), Box<dyn Error>> {
    // Load configuration based on the platform
    let platform_config = config::load_config()?;
    
    // Initialize the connection manager
    let mut connection_manager = connection::manager::ConnectionManager::new(platform_config);
    
    println!("Starting data receiver...");

    loop {
        // Attempt to receive data from the current connection
        match connection_manager.receive_data() {
            Ok(data) => {
                // Process the received data (e.g., store in database)
                database::store_data(data)?;
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                // Attempt to switch connection if an error occurs
                connection_manager.switch_connection()?;
            }
        }

        // Sleep for a short duration to prevent busy waiting
        thread::sleep(Duration::from_millis(100));
    }
}