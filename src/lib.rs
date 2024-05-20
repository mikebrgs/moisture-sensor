// Local modules
mod i2c;

// Local imports
pub use i2c::Address;

// Public imports
use embedded_hal::i2c::I2c;

#[derive(Debug)]

pub enum MoistureSensorError {
    ConversionError(String),
    IOError(String),
    OtherError
}


pub struct MoistureSensor<I2C> {
    dev: i2c::MoistureSensorI2c<I2C>
}

impl<I2C: I2c> MoistureSensor<I2C> {
    
    pub fn new(dev: I2C, address: Address) -> MoistureSensor<I2C> {
        let wrapper = i2c::MoistureSensorI2c::new(dev, address.into());
        MoistureSensor { dev: wrapper }
    }

    pub fn build(dev: I2C, address: Address) -> MoistureSensor<I2C> {
        let sensor = MoistureSensor::new(dev, address.into());
        sensor
    }

    pub fn get_moisture_level(&mut self) -> Result<u16, MoistureSensorError> {
        self.dev.get_moisture_level().map_err(
            |err| match err {
                i2c::MoistureSensorI2cError::IOError(m) => MoistureSensorError::IOError(m),
            }
        )
    }

    pub fn turn_on_led(&mut self) -> Result<(), MoistureSensorError> {
        self.dev.set_led(i2c::Led::On).map_err(
            |err| match err {
                i2c::MoistureSensorI2cError::IOError(m) => MoistureSensorError::IOError(m),
            }
        )
    }

    pub fn turn_off_led(&mut self) -> Result<(), MoistureSensorError> {
        self.dev.set_led(i2c::Led::Off).map_err(
            |err| match err {
                i2c::MoistureSensorI2cError::IOError(m) => MoistureSensorError::IOError(m),
            }
        )
    }

}


#[cfg(test)]
mod tests {
    // Local imports
    use super::*;

    // Public imports
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    /// Test high level moisture value
    /// Simple test to extract the value
    #[test]
    fn read_moisture_value() {
        let address: u8 = i2c::Address::Default.into();
        let expectations = [
            I2cTransaction::write_read(address, vec![i2c::constants::registers::COMMAND_GET_VALUE], vec![0x00, 0x00]),
        ];
        let i2c = I2cMock::new(&expectations);
        let mut i2c_clone = i2c.clone();

        let mut moisture_sensor = MoistureSensor::build(i2c, Address::Default);
        let moisture = moisture_sensor.get_moisture_level().unwrap();

        assert_eq!(moisture, 0);
        i2c_clone.done();
    }

}