mod light;
mod bmp180;
mod dht11;

use alloc::vec::Vec;
use async_trait::async_trait;
use embassy_rp::i2c::I2c;
use embassy_time::Delay;

#[async_trait]
pub trait WeatherSensor {
    async fn read(&mut self) -> Result<String, &'static str>;
}

pub async fn init_all<'a>(
    _i2c: &'a mut I2c<'a, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Async>,
    _delay: &'a mut Delay,
) -> Vec<Box<dyn WeatherSensor + 'a>> {
    let mut sensors: Vec<Box<dyn WeatherSensor>> = Vec::new();
    // Add real sensor instances here
    // sensors.push(Box::new(light::LightSensor::new(...)));
    sensors
}
