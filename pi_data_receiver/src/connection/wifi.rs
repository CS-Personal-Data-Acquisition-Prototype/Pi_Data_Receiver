use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::error::Error;

pub struct WifiConnection {
    listener: TcpListener,
}

impl WifiConnection {
    pub fn new(address: &str) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(address)?;
        Ok(WifiConnection { listener })
    }

    pub fn accept_connection(&self) -> Result<TcpStream, Box<dyn Error>> {
        let (stream, addr) = self.listener.accept()?;
        println!("Wi-Fi client connected: {:?}", addr);
        Ok(stream)
    }

    pub fn receive_data(&self, stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
        let mut reader = BufReader::new(stream);
        let mut data = String::new();
        reader.read_line(&mut data)?;
        Ok(data)
    }

    pub fn send_data(&self, stream: &mut TcpStream, data: &str) -> Result<(), Box<dyn Error>> {
        stream.write_all(data.as_bytes())?;
        Ok(())
    }
}