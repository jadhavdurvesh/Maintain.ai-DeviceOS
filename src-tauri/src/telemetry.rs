use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryReading {
    pub sensor_id: String,
    pub parameter_id: String,
    pub value: f64,
    pub unit: String,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEnvelope {
    pub protocol: String,
    pub protocol_version: String,
    pub device_id: Option<String>,
    pub configuration_id: String,
    pub sequence: u64,
    pub readings: Vec<TelemetryReading>,
}

pub fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or_default()
}

pub fn create_envelope(configuration_id: String, device_id: Option<String>, sequence: u64, readings: Vec<TelemetryReading>) -> TelemetryEnvelope {
    TelemetryEnvelope {
        protocol: "maintain-ai-telemetry".into(),
        protocol_version: "1.0".into(),
        device_id,
        configuration_id,
        sequence,
        readings,
    }
}

pub fn encode_json(envelope: &TelemetryEnvelope) -> Result<String, String> {
    serde_json::to_string(envelope).map_err(|e| e.to_string())
}
