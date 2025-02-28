use std::error::Error;
use std::time::Duration;
use serialport::SerialPort;
use crate::connection::manager::ConnectionManager;

pub struct PiZero {
    connection_manager: ConnectionManager,
}

impl PiZero {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let connection_manager = ConnectionManager::new();
        Ok(PiZero { connection_manager })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.connection_manager.check_connections()?;
            // Add logic to handle data reception and processing
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn setup_usb(&self) -> Result<(), Box<dyn Error>> {
        // Setup USB connection specifics for Raspberry Pi Zero
        // Example using builder pattern instead of SerialPortSettings
        // Replace "/dev/ttyUSB0" with your actual port name
        let _port = serialport::new("/dev/ttyUSB0", 9600)
            .timeout(Duration::from_secs(1))
            .open()?;
        
        // Initialize USB port logic here
        Ok(())
    }

    pub fn setup_wifi(&self) -> Result<(), Box<dyn Error>> {
        // Setup Wi-Fi connection specifics for Raspberry Pi Zero
        // Initialize Wi-Fi connection here
        Ok(())
    }
}