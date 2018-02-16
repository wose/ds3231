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

/// Date and Time
#[derive(Copy, Clone)]
pub struct DateTime {
    pub sec: u8,
    pub min: u8,
    pub hour: u8,
    pub mday: u8,
    pub mon: u8,
    pub year: u16,
    pub wday: u8,
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

    /// Get the current date and time.
    pub fn get_datetime(&mut self) -> Result<DateTime, E> {
        let data = self.read_dt()?;
        Ok(
            DateTime {
                sec: bcd2dec(data[0]),
                min: bcd2dec(data[1]),
                hour: bcd2dec(data[2]),
                mday: bcd2dec(data[4]),
                mon: bcd2dec(data[5] & 0x1F),
                wday: bcd2dec(data[3]),
                year: bcd2dec(data[6]) as u16,
            }
        )
    }

    /// Set the date and time.
    pub fn set_datetime(&mut self, dt: DateTime) -> Result<(), E> {
        let mut data = [0u8; 8];
        let (yy, century) = match dt.year {
            year if year >= 2000 => (year - 2000, 0x80),
            year => (year - 1900, 0x00),
        };

        data[0] = Register::Seconds.addr();
        data[1] = dec2bcd(dt.sec);
        data[2] = dec2bcd(dt.min);
        data[3] = dec2bcd(dt.hour);
        data[4] = dec2bcd(dt.wday);
        data[5] = dec2bcd(dt.mday);
        data[6] = dec2bcd(dt.mon) | century;
        data[7] = dec2bcd(yy as u8);

        self.write_dt(&data)?;

        Ok(())
    }

    fn read_dt(&mut self) -> Result<[u8; 7], E> {
        let mut buffer = [0u8; 7];
        self.i2c.write_read(ADDRESS, &[Register::Seconds.addr()], &mut buffer)?;
        Ok(buffer)
    }

    fn write_dt(&mut self, data: &[u8]) -> Result<(), E> {
        self.i2c.write(ADDRESS, data)
    }

    fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        let mut buffer = [0];
        self.i2c.write_read(ADDRESS, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }
}

fn dec2bcd(value: u8) -> u8 {
    (value / 10 * 16) + (value % 10)
}

fn bcd2dec(value: u8) -> u8 {
    (value / 16 * 10) + (value % 16)
}
