use std::fs;
use std::path::Path;
use toml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub usb: UsbConfig,
    pub wifi: WifiConfig,
}

#[derive(Debug, Deserialize)]
pub struct UsbConfig {
    pub port: String,
    pub baud_rate: u32,
}

#[derive(Debug, Deserialize)]
pub struct WifiConfig {
    pub ssid: String,
    pub password: String,
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