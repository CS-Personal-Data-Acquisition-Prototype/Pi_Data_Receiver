use std::fs;
use std::path::Path;
use toml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default: DefaultConfig,
    pub usb: UsbConfig,
    pub wifi: WifiConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
pub struct DefaultConfig {
    pub connection_type: String,
    pub usb_device_path: String,
    pub wifi_ssid: String,
    pub wifi_password: String,
    pub data_receive_timeout: u32,
    pub retry_interval: u32,
    pub max_retries: u32,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub log_level: String,
    pub log_file: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct UsbConfig {
    pub port: String,
    pub vendor_id: String,
    pub product_id: String,
    pub buffer_size: u32,
    pub baud_rate: u32,
}

#[derive(Debug, Deserialize)]
pub struct WifiConfig {
    pub ssid: String,
    pub password: String,
    pub retry_interval: u32, 
    pub ip_address: String,
}

impl Config {
    pub fn load(platform: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = format!("config/{}.toml", platform);
        let config_str = fs::read_to_string(&config_path)?;
        let config: Config = from_str(&config_str)?;
        Ok(config)
    }
}