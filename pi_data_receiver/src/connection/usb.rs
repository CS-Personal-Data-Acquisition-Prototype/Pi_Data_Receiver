use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::time::Duration;

// Conditional import of serialport
#[cfg(target_os = "linux")]
use serialport::SerialPort;

// Real implementation for Linux systems
#[cfg(target_os = "linux")]
pub struct UsbConnection {
    port: Box<dyn SerialPort>,
}

#[cfg(target_os = "linux")]
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

// Mock implementation for non-Linux systems (macOS, Windows)
#[cfg(not(target_os = "linux"))]
pub struct UsbConnection {
    port_name: String,
}

#[cfg(not(target_os = "linux"))]
impl UsbConnection {
    pub fn new(port_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Mock USB: Creating connection to {}", port_name);
        Ok(UsbConnection { port_name: port_name.to_string() })
    }

    pub fn read_data(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        println!("Mock USB: Reading data from {}", self.port_name);
        Ok("MOCK DATA: temperature=25.5,humidity=60.0".to_string())
    }

    pub fn set_timeout(&mut self, timeout: Duration) -> Result<(), Box<dyn std::error::Error>> {
        println!("Mock USB: Setting timeout to {:?}", timeout);
        Ok(())
    }
}

// Functions with conditional implementations
#[cfg(target_os = "linux")]
pub fn list_available_ports() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let ports = serialport::available_ports()?;
    let port_names = ports.into_iter().map(|p| p.port_name).collect();
    Ok(port_names)
}

#[cfg(not(target_os = "linux"))]
pub fn list_available_ports() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Return mock port names
    Ok(vec!["/dev/ttyUSB0".to_string(), "/dev/ttyACM0".to_string()])
}

#[cfg(target_os = "linux")]
pub fn is_connected() -> Result<bool, Box<dyn std::error::Error>> {
    let available_ports = serialport::available_ports()?;
    Ok(!available_ports.is_empty())
}

#[cfg(not(target_os = "linux"))]
pub fn is_connected() -> Result<bool, Box<dyn std::error::Error>> {
    // Always return true for the mock implementation
    println!("Mock USB: Checking connection status");
    Ok(true)
}

#[cfg(target_os = "linux")]
pub fn connect() -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to connect to USB device...");
    let ports = serialport::available_ports()?;
    
    if ports.is_empty() {
        return Err("No serial ports found".into());
    }
    
    let port_info = &ports[0];
    println!("Connecting to {}", port_info.port_name);
    println!("USB connection established");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn connect() -> Result<(), Box<dyn std::error::Error>> {
    println!("Mock USB: Attempting to connect to USB device...");
    println!("Mock USB: Connection established");
    Ok(())
}