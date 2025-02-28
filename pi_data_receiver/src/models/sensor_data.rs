struct SensorData {
    session_id: Option<i32>,
    timestamp: String,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    accel_x: f64,
    accel_y: f64,
    accel_z: f64,
    gyro_x: f64,
    gyro_y: f64,
    gyro_z: f64,
    dac_1: f64,
    dac_2: f64,
    dac_3: f64,
    dac_4: f64,
}

impl SensorData {
    pub fn new(
        session_id: Option<i32>,
        timestamp: String,
        latitude: f64,
        longitude: f64,
        altitude: f64,
        accel_x: f64,
        accel_y: f64,
        accel_z: f64,
        gyro_x: f64,
        gyro_y: f64,
        gyro_z: f64,
        dac_1: f64,
        dac_2: f64,
        dac_3: f64,
        dac_4: f64,
    ) -> Self {
        SensorData {
            session_id,
            timestamp,
            latitude,
            longitude,
            altitude,
            accel_x,
            accel_y,
            accel_z,
            gyro_x,
            gyro_y,
            gyro_z,
            dac_1,
            dac_2,
            dac_3,
            dac_4,
        }
    }

    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            self.session_id
                .map(|s| s.to_string())
                .unwrap_or("None".to_string()),
            self.timestamp,
            self.latitude,
            self.longitude,
            self.altitude,
            self.accel_x,
            self.accel_y,
            self.accel_z,
            self.gyro_x,
            self.gyro_y,
            self.gyro_z,
            self.dac_1,
            self.dac_2,
            self.dac_3,
            self.dac_4
        )
    }
}