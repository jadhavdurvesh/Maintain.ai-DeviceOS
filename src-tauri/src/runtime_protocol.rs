use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::config_store::DeviceConfiguration;
use crate::registry::sensors;
use crate::telemetry::{create_envelope, TelemetryEnvelope, TelemetryReading};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeReading {
    pub sensor_id: String,
    pub parameter_id: String,
    pub value: f64,
    pub unit: String,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeFrame {
    pub protocol: String,
    pub protocol_version: String,
    pub device_id: Option<String>,
    pub configuration_id: String,
    pub sequence: u64,
    pub readings: Vec<RuntimeReading>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn validate_frame(config: &DeviceConfiguration, frame: &RuntimeFrame) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    if frame.protocol != "maintain-ai-telemetry" { errors.push(format!("Unsupported runtime protocol: {}", frame.protocol)); }
    if frame.protocol_version != "1.0" { errors.push(format!("Unsupported telemetry protocol version: {}", frame.protocol_version)); }
    if frame.configuration_id != config.id { errors.push("Frame configuration does not match the active DeviceOS configuration".into()); }
    if let Some(expected_device_id) = config.device_id.as_deref() {
        if frame.device_id.as_deref() != Some(expected_device_id) {
            errors.push("Frame device_id does not match the active DeviceOS configuration".into());
        }
    }
    if frame.readings.is_empty() {
        warnings.push("Telemetry frame contains no readings.".into());
    }

    let allowed: HashSet<(&str, &str)> = config.sensors.iter()
        .flat_map(|s| s.parameter_ids.iter().map(move |p| (s.sensor_id.as_str(), p.as_str())))
        .collect();

    for reading in &frame.readings {
        if !reading.value.is_finite() {
            errors.push(format!("Reading {} / {} contains a non-finite value", reading.sensor_id, reading.parameter_id));
            continue;
        }
        if !allowed.contains(&(reading.sensor_id.as_str(), reading.parameter_id.as_str())) {
            errors.push(format!("Reading {} / {} is not enabled by the active configuration", reading.sensor_id, reading.parameter_id));
            continue;
        }
        if let Some(sensor) = sensors().into_iter().find(|s| s.id == reading.sensor_id) {
            if let Some(parameter) = sensor.parameters.iter().find(|p| p.id == reading.parameter_id) {
                if parameter.unit != reading.unit { warnings.push(format!("Unit mismatch for {} / {}: expected {}, received {}", reading.sensor_id, reading.parameter_id, parameter.unit, reading.unit)); }
            }
        }
    }

    ValidationResult { valid: errors.is_empty(), errors, warnings }
}

pub fn to_telemetry(config: &DeviceConfiguration, frame: &RuntimeFrame) -> Result<TelemetryEnvelope, String> {
    let result = validate_frame(config, frame);
    if !result.valid { return Err(result.errors.join("; ")); }
    let readings = frame.readings.iter().map(|r| TelemetryReading {
        sensor_id: r.sensor_id.clone(), parameter_id: r.parameter_id.clone(), value: r.value, unit: r.unit.clone(), timestamp_ms: r.timestamp_ms,
    }).collect();
    Ok(create_envelope(config.id.clone(), frame.device_id.clone(), frame.sequence, readings))
}
