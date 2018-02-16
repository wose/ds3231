//! A platform agnostic driver to interface with the DS3231 real-time clock.
//!
//!

//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::i2c::{Read, Write, WriteRead};

/// I2C address
pub const ADDRESS: u8 = 0x68;

/// Clock registers
#[allow(dead_code)]
#[derive(Copy, Clone)]
enum Register {
    Seconds = 0x00,
    Minutes = 0x01,
    Hours = 0x02,
    DayOfWeek = 0x03,
    DayOfMonth = 0x04,
    MonthCentury = 0x05,
    Year = 0x06,
    Alarm1Seconds = 0x07,
    Alarm1Minutes = 0x08,
    Alarm1Hours = 0x09,
    Alarm1Day = 0x0A,
    Alarm2Minutes = 0x0B,
    Alarm2Hour = 0x0C,
    Alarm2Day = 0x0D,
    Control = 0x0E,
    Status = 0x0F,
    AgingOffset = 0x10,
    TempMSB = 0x11,
    TempLSB = 0x12,
}

impl Register {
    /// Get register address.
    fn addr(&self) -> u8 {
        *self as u8
    }
}

/// DS3231 Driver
pub struct DS3231<I2C> {
    i2c: I2C,
}

impl<I2C, E> DS3231<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    /// Creates a new driver from an I2C peripheral.
    pub fn new(i2c: I2C) -> Self {
        DS3231 { i2c }
    }

    pub fn temperature(&mut self) -> Result<f32, E> {
        let mut temperature = self.read_register(Register::TempMSB)? as f32;
        temperature += (self.read_register(Register::TempLSB)? >> 6) as f32 * 0.25;
        Ok(temperature)
    }

    fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        let mut buffer = [0];
        self.i2c.write_read(ADDRESS, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }
}
