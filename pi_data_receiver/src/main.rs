use std::error::Error;
use std::thread;
use std::time::Duration;

mod config;
mod connection;
mod database;
mod models;
mod platform;

fn main() -> Result<(), Box<dyn Error>> {
    // Determine which platform we're running on (e.g., "pi_zero", "pi_five")
    // In a real app, you might detect this automatically
    let platform_name = "pi_zero"; // or "pi_five"
    
    // Load configuration for the specific platform
    let config = config::Config::load(platform_name)?;
    
    // Initialize the connection manager
    let mut connection_manager = connection::manager::ConnectionManager::new();
    
    println!("Starting data receiver...");
    println!("USB Port: {}", config.usb.port);
    println!("WiFi SSID: {}", config.wifi.ssid);

    loop {
        // Check connections
        connection_manager.check_connections()?;
        
        // Sleep to prevent busy waiting
        thread::sleep(Duration::from_millis(100));
    }
}