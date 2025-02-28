# pi_data_receiver/pi_data_receiver/README.md

# Pi Data Receiver

This project is designed to allow a Raspberry Pi to receive data from USB and Wi-Fi connections. It is initially targeted for the Raspberry Pi Zero, with plans to adapt it for the Raspberry Pi 5 in the future.

## Features

- **Dual Connection Support**: Automatically switch between USB and Wi-Fi connections based on availability.
- **Data Reception**: Efficiently receive and process sensor data from connected devices.
- **Database Integration**: Store received data in a SQLite database for later analysis.
- **Platform-Specific Implementations**: Tailored configurations and functionalities for Raspberry Pi Zero and Raspberry Pi 5.

## Project Structure

- `src/main.rs`: Entry point of the application.
- `src/config.rs`: Handles configuration settings.
- `src/connection/`: Manages USB and Wi-Fi connections.
- `src/database/`: Manages database interactions.
- `src/models/`: Defines data models for the application.
- `src/platform/`: Contains platform-specific implementations.
- `config/`: Configuration files for different platforms.
- `tests/`: Unit and integration tests for the application.

## Setup Instructions

1. **Clone the Repository**:
   ```bash
   git clone <repository-url>
   cd pi_data_receiver
   ```

2. **Install Dependencies**:
   Ensure you have Rust and Cargo installed. Then run:
   ```bash
   cargo build
   ```

3. **Configuration**:
   Modify the configuration files in the `config/` directory as needed for your specific platform.

4. **Run the Application**:
   Execute the following command to start the application:
   ```bash
   cargo run
   ```

## Usage

The application will automatically attempt to establish a connection via USB first. If the USB connection is lost, it will switch to Wi-Fi. Ensure that your Raspberry Pi is properly configured for both connection types.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.