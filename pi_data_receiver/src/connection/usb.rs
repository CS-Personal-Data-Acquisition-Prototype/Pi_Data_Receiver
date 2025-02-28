use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::time::Duration;
use serialport::SerialPort;

pub struct UsbConnection {
    port: Box<dyn SerialPort>,
}

impl UsbConnection {
    pub fn new(port_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Using the builder pattern with a default baud rate of 9600
        let port = serialport::new(port_name, 9600).open()?;
        Ok(UsbConnection { port })
    }

    pub fn read_data(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buffer: Vec<u8> = vec![0; 1024];
        let bytes_read = self.port.read(&mut buffer)?;
        let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(data)
    }

    pub fn set_timeout(&mut self, timeout: Duration) -> Result<(), Box<dyn std::error::Error>> {
        self.port.set_timeout(timeout)?;
        Ok(())
    }
}

pub fn list_available_ports() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let ports = serialport::available_ports()?;
    let port_names = ports.into_iter().map(|p| p.port_name).collect();
    Ok(port_names)
}