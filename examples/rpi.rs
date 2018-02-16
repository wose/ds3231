extern crate linux_embedded_hal as hal;
extern crate chrono;
extern crate ds3231;

use chrono::prelude::*;

use hal::{I2cdev};
use ds3231::{DS3231};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut ds3231 = DS3231::new(dev);

    let now: DateTime<Local> = Local::now();
    let _now = ds3231::DateTime {
        sec: now.second() as u8,
        min: now.minute() as u8,
        hour: now.hour() as u8,
        mday: now.day() as u8,
        mon: now.month() as u8,
        wday: now.weekday().number_from_sunday() as u8,
        year: now.year() as u16,
    };

    // uncomment to set the date and time
    //ds3231.set_datetime(_now).unwrap();

    let dt = ds3231.get_datetime().unwrap();
    println!("{}-{:02}-{:02} {:02}:{:02}:{:02}", dt.year, dt.mon, dt.mday, dt.hour, dt.min, dt.sec);

    // The RTC contains a temperature compensated crystal oscillator.
    let temperature = ds3231.temperature().unwrap();
    println!("Temperature: {} °C", temperature);
}
