use serde::{Deserialize, Serialize};

use crate::telemetry::{now_ms, TelemetryReading};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawSample {
    Digital(bool),
    Analog(u16),
    Scalar(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeChannel {
    pub sensor_id: String,
    pub parameter_id: String,
    pub unit: String,
    pub sample_period_ms: u32,
    pub scale: f64,
    pub offset: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeProcessor {
    pub channels: Vec<RuntimeChannel>,
}

impl RuntimeProcessor {
    pub fn process(&self, sensor_id: &str, parameter_id: &str, sample: RawSample) -> Result<TelemetryReading, String> {
        let channel = self.channels.iter().find(|c| c.sensor_id == sensor_id && c.parameter_id == parameter_id)
            .ok_or_else(|| format!("Runtime channel not configured: {}/{}", sensor_id, parameter_id))?;
        let raw = match sample {
            RawSample::Digital(value) => if value { 1.0 } else { 0.0 },
            RawSample::Analog(value) => value as f64,
            RawSample::Scalar(value) => value,
        };
        Ok(TelemetryReading {
            sensor_id: sensor_id.into(),
            parameter_id: parameter_id.into(),
            value: raw * channel.scale + channel.offset,
            unit: channel.unit.clone(),
            timestamp_ms: now_ms(),
        })
    }
}
