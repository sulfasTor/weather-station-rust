use super::WeatherSensor;
use async_trait::async_trait;

pub struct LightSensor {}

impl LightSensor {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl WeatherSensor for LightSensor {
    async fn read(&mut self) -> Result<String, &'static str> {
        Ok("Light: 75%".to_string())
    }
}
