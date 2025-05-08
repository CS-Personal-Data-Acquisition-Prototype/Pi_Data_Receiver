# Pi Data Receiver

This project is designed to allow a Raspberry Pi to receive data from USB and Wi-Fi connections. It is initially targeted for the Raspberry Pi Zero, with plans to adapt it for the Raspberry Pi 5 in the future.

## Features

- **Dual Connection Support**: Automatically switch between USB and Wi-Fi connections based on availability.
- **Data Reception**: Efficiently receive and process sensor data from connected devices.
- **Database Integration**: Store received data in a SQLite database for later analysis.
- **Platform-Specific Implementations**: Tailored configurations and functionalities for Raspberry Pi Zero and Raspberry Pi 5.

## Project Structure

- `pi_data_receiver/`: Folder containing the following files.
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

## License Notice
To apply the Apache License to your work, attach the following boilerplate notice. The text should be enclosed in the appropriate comment syntax for the file format. We also recommend that a file or class name and description of purpose be included on the same "printed page" as the copyright notice for easier identification within third-party archives.

    Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
    Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
    
    http://www.apache.org/licenses/LICENSE-2.0
    Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
