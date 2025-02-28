// filepath: /pi_data_receiver/pi_data_receiver/tests/wifi_tests.rs
use std::net::{TcpListener, TcpStream};
use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wifi_connection_establishment() {
        // Simulate establishing a Wi-Fi connection
        let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
        let _ = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("Failed to accept connection");
            let response = b"Hello from Wi-Fi!";
            stream.write_all(response).expect("Failed to write to stream");
        });

        // Simulate a client connecting to the Wi-Fi server
        thread::sleep(Duration::from_secs(1)); // Wait for the server to start
        let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to server");
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

        assert_eq!(response, "Hello from Wi-Fi!");
    }

    #[test]
    fn test_wifi_data_reception() {
        // Simulate data reception over Wi-Fi
        let listener = TcpListener::bind("127.0.0.1:7879").expect("Failed to bind to address");
        let _ = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("Failed to accept connection");
            let data = b"Sensor data: 123.45";
            stream.write_all(data).expect("Failed to write to stream");
        });

        // Simulate a client connecting to the Wi-Fi server
        thread::sleep(Duration::from_secs(1)); // Wait for the server to start
        let mut stream = TcpStream::connect("127.0.0.1:7879").expect("Failed to connect to server");
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
        let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);

        assert_eq!(received_data, "Sensor data: 123.45");
    }
}