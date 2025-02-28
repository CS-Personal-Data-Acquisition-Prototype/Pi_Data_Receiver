use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use rusqlite::{params, Connection};
use std::error::Error;
use std::time::Duration;
use socket2::{Socket, Domain, Type};
use serialport::SerialPort;

pub struct PiFiveConnectionManager {
    usb_connected: bool,
    wifi_connected: bool,
}

impl PiFiveConnectionManager {
    pub fn new() -> Self {
        Self {
            usb_connected: false,
            wifi_connected: false,
        }
    }

    pub fn check_connections(&mut self) -> Result<(), Box<dyn Error>> {
        // Logic to check USB connection
        self.usb_connected = self.check_usb_connection()?;
        // Logic to check Wi-Fi connection
        self.wifi_connected = self.check_wifi_connection()?;
        Ok(())
    }

    fn check_usb_connection(&self) -> Result<bool, Box<dyn Error>> {
        // Implement USB connection checking logic here
        Ok(false) // Placeholder
    }

    fn check_wifi_connection(&self) -> Result<bool, Box<dyn Error>> {
        // Implement Wi-Fi connection checking logic here
        Ok(false) // Placeholder
    }

    pub fn switch_connection(&mut self) -> Result<(), Box<dyn Error>> {
        if self.usb_connected {
            println!("Using USB connection...");
            // Logic to handle USB data reception
        } else if self.wifi_connected {
            println!("Using Wi-Fi connection...");
            // Logic to handle Wi-Fi data reception
        } else {
            println!("No active connections.");
        }
        Ok(())
    }
}