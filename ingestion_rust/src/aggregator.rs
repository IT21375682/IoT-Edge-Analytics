pub struct Aggregator {
    temperature_sum: f64,
    humidity_sum: f64,
    count: u32,
}

impl Aggregator {
    pub fn new() -> Self {
        Aggregator {
            temperature_sum: 0.0,
            humidity_sum: 0.0,
            count: 0,
        }
    }

    pub fn aggregate(&mut self, data: &TelemetryData) {
        self.temperature_sum += data.temperature;
        self.humidity_sum += data.humidity;
        self.count += 1;
    }

    pub fn get_aggregated(&self) -> TelemetryData {
        TelemetryData {
            temperature: self.temperature_sum / self.count as f64,
            humidity: self.humidity_sum / self.count as f64,
        }
    }

    pub fn should_send(&self) -> bool {
        self.count >= 10  // Send data when 10 readings are aggregated
    }
}
