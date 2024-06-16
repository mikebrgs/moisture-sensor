// Local modules

// Local imports

// Local-ish imports
use moisture_sensor;

// Public imports
use linux_embedded_hal::{Delay, I2cdev};
use std::{fs::File, io::BufReader, thread, time};
use chrono::{DateTime, Utc};


fn main() {
    // Set logger
    env_logger::init();

    // Load sensors
    let mut moisture_sensor = moisture_sensor::MoistureSensor::build(
        I2cdev::new("/dev/i2c-1").expect("Failed to connect to moisture sensor"),
        moisture_sensor::Address::Default
    );

    // Wait for 1 second to get the connections setup
    thread::sleep(time::Duration::from_secs(1));

    loop {
        // Timestamp creation
        let timestamp = time::SystemTime::now();
        let timestamp: DateTime<Utc> = timestamp.into();

        // Generate sensor values
        let moisture_level = moisture_sensor.get_moisture_level().unwrap() as f32;

        println!("[{}] moisture level: {}", timestamp, moisture_level);


        thread::sleep(time::Duration::from_secs(1))
    }
}
