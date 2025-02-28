use std::error::Error;
use std::time::{Duration, Instant};
use std::thread;

use crate::connection::{usb, wifi};

pub struct ConnectionManager {
    usb_connected: bool,
    wifi_connected: bool,
    last_usb_check: Instant,
    last_wifi_check: Instant,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            usb_connected: false,
            wifi_connected: false,
            last_usb_check: Instant::now(),
            last_wifi_check: Instant::now(),
        }
    }

    pub fn check_connections(&mut self) -> Result<(), Box<dyn Error>> {
        if self.usb_connected {
            if !usb::is_connected()? {
                println!("USB connection lost. Switching to Wi-Fi...");
                self.usb_connected = false;
                self.switch_to_wifi()?;
            }
        } else {
            if usb::is_connected()? {
                println!("USB connection established.");
                self.usb_connected = true;
                self.wifi_connected = false; // Disable Wi-Fi if USB is connected
            }
        }

        if self.wifi_connected {
            if !wifi::is_connected()? {
                println!("Wi-Fi connection lost. Checking USB...");
                self.wifi_connected = false;
                self.switch_to_usb()?;
            }
        } else {
            if wifi::is_connected()? {
                println!("Wi-Fi connection established.");
                self.wifi_connected = true;
                self.usb_connected = false; // Disable USB if Wi-Fi is connected
            }
        }

        Ok(())
    }

    fn switch_to_wifi(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.wifi_connected {
            wifi::connect()?;
            self.wifi_connected = true;
        }
        Ok(())
    }

    fn switch_to_usb(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.usb_connected {
            usb::connect()?;
            self.usb_connected = true;
        }
        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.check_connections() {
                eprintln!("Error checking connections: {}", e);
            }
            thread::sleep(Duration::from_secs(5)); // Check every 5 seconds
        }
    }
}