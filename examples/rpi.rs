extern crate linux_embedded_hal as hal;
extern crate ds3231;

//use std::thread;
//use std::time::Duration;

use hal::{I2cdev};
use ds3231::DS3231;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut ds3231 = DS3231::new(dev);

    // The RTC contains a temperature compensated crystal oscillator.
    for _ in 0..5 {
        let temperature = ds3231.temperature().unwrap();
        println!("Temperature: {} °C", temperature);
    }
}
