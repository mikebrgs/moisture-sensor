// Local modules
pub mod constants;

// Local imports

// Public imports
use embedded_hal::i2c::I2c;
use byteorder::{BigEndian, ByteOrder};


#[derive(Debug)]
pub enum MoistureSensorI2cError {
    IOError(String)
}

pub enum Led {
    Off=0,
    On=1
}

impl From<u8> for Led {
    fn from(item: u8) -> Self {
        match item {
            0 => Self::Off,
            1 => Self::On,
            _ => panic!("Not expected")
        }
    }
}

impl From<Led> for u8 {
    fn from(item: Led) -> u8 {
        match item {
            Led::Off => 0,
            Led::On => 1,
        }
    }
}

pub enum ErrorStatus {
    Off=0,
    On=1
}

impl From<u8> for ErrorStatus {
    fn from(item: u8) -> Self {
        match item {
            0 => Self::Off,
            1 => Self::On,
            _ => panic!("Not expected")
        }
    }
}

impl From<ErrorStatus> for u8 {
    fn from(item: ErrorStatus) -> u8 {
        match item {
            ErrorStatus::Off => 0,
            ErrorStatus::On => 1,
        }
    }
}

pub enum Address {
    Default
}

impl From<Address> for u8 {
    fn from(item: Address) -> u8 {
        match item {
            Address::Default => constants::addresses::DEFAULT
        }
    }
}


/// I2C wrapper for MoistureSensor
pub struct MoistureSensorI2c<I2C>{
    i2c: I2C,
    address: u8
}


impl<I2C: I2c> MoistureSensorI2c<I2C>{

    /// Create new MoistureSensorI2c.
    pub fn new(i2c: I2C, address: u8) -> MoistureSensorI2c<I2C> {
        MoistureSensorI2c { i2c, address }
    }

    pub fn get_moisture_level(&mut self) -> Result<u16, MoistureSensorI2cError> {
        let mut buffer = [0u8; 2];
        let result = read_from_register(
            self,
            constants::registers::COMMAND_GET_VALUE, &mut buffer
        );
    
        match result {
            Ok(()) => Ok(BigEndian::read_u16(&buffer)),
            Err(_) => Err(MoistureSensorI2cError::IOError(String::from("Error reading moisture level.")))
        }
    }

    pub fn set_led(&mut self, led: Led) -> Result<(), MoistureSensorI2cError> {
        let result = write_to_register(self, led.into(), &[]);
        
        match result {
            Ok(()) => Ok(()),
            Err(_) => Err(MoistureSensorI2cError::IOError(String::from("Error setting the LED.")))
        }
    }

    pub fn get_error_status(&mut self) -> Result<ErrorStatus, MoistureSensorI2cError> {
        let mut buffer = [0u8];
        let result = read_from_register(self, constants::registers::SENSOR_STATUS, &mut buffer);
    
        match result {
            Ok(()) => Ok((buffer.first().unwrap() & 0x01).into()),
            Err(_) => Err(MoistureSensorI2cError::IOError(String::from("Error reading error status.")))
        }

    }

    pub fn set_address(&mut self, address: u8) -> Result<(), MoistureSensorI2cError> {
        write_to_register(
                self, constants::registers::COMMAND_CHANGE_ADDRESS,
                &[address]
            )
            .map_err(|_| MoistureSensorI2cError::IOError(String::from("Error setting address")))
    }

}


/// Get value from a specific register in sensor.
pub fn read_from_register<I2C: I2c>(dev: &mut MoistureSensorI2c<I2C> , register: u8, buffer: &mut [u8]) -> Result<(), MoistureSensorI2cError> {
    match dev.i2c.write_read(dev.address, &[register], buffer) {
        Ok(_) => Ok(()),
        Err(_) => Err(MoistureSensorI2cError::IOError(String::from("Error reading from register.")))
    }
}

/// Set value from a specific register in sensor.
pub fn write_to_register<I2C: I2c>(dev: &mut MoistureSensorI2c<I2C>, register: u8, bytes: &[u8]) -> Result<(), MoistureSensorI2cError> {
    let mut buffer = Vec::<u8>::with_capacity(1+bytes.len());
    buffer.push(register);
    for value in bytes {
        buffer.push(*value);
    }
    // TODO check if it matches write_bytes
    match dev.i2c.write(dev.address, &buffer) {
        Ok(_) => Ok(()),
        Err(_) => Err(MoistureSensorI2cError::IOError(String::from("Error writing to register.")))
    }
}


#[cfg(test)]
mod tests {
    // Local import
    use super::*;

    // Public import
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    #[test]
    fn read_moisture_value() {
        let address: u8 = Address::Default.into();
        let expectations = [
            I2cTransaction::write_read(address, vec![constants::registers::COMMAND_GET_VALUE], vec![0x00, 0x00]),
        ];
        let i2c = I2cMock::new(&expectations);
        let mut i2c_clone = i2c.clone();

        let mut moisture_sensor = MoistureSensorI2c::new(i2c, constants::addresses::DEFAULT);
        let moisture = moisture_sensor.get_moisture_level().unwrap();

        assert_eq!(moisture, 0);
        i2c_clone.done();
    }

    #[test]
    fn set_led_on_and_off() {
        let address: u8 = Address::Default.into();
        let expectations = [
            I2cTransaction::write(address, vec![0x00]),
            I2cTransaction::write(address, vec![0x01]),
        ];
        let i2c = I2cMock::new(&expectations);
        let mut i2c_clone = i2c.clone();

        let mut moisture_sensor = MoistureSensorI2c::new(i2c, constants::addresses::DEFAULT);
        moisture_sensor.set_led(Led::Off).unwrap();
        moisture_sensor.set_led(Led::On).unwrap();
        i2c_clone.done();
    }

    #[test]
    fn read_error_status() {
        let address: u8 = Address::Default.into();
        let expectations = [
            I2cTransaction::write_read(address, vec![constants::registers::SENSOR_STATUS], vec![0x01]),
        ];
        let i2c = I2cMock::new(&expectations);
        let mut i2c_clone = i2c.clone();

        let mut moisture_sensor = MoistureSensorI2c::new(i2c, constants::addresses::DEFAULT);
        let error_status: u8 = moisture_sensor.get_error_status().unwrap().into();

        assert_eq!(error_status, 1);
        i2c_clone.done();

    }

    #[test]
    fn change_i2c_address() {
        let address: u8 = Address::Default.into();
        let expectations = [
            I2cTransaction::write(address, vec![constants::registers::COMMAND_CHANGE_ADDRESS, 0x50]),
        ];
        let i2c = I2cMock::new(&expectations);
        let mut i2c_clone = i2c.clone();

        let mut moisture_sensor = MoistureSensorI2c::new(i2c, constants::addresses::DEFAULT);
        moisture_sensor.set_address(0x50).unwrap();
        i2c_clone.done();
    }
}