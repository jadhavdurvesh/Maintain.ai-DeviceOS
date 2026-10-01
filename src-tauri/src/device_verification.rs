use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};
use crate::config_store::DeviceConfiguration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub success: bool,
    pub port: String,
    pub ready_seen: bool,
    pub readings_seen: usize,
    pub expected_readings: usize,
    pub valid_readings: usize,
    pub message: String,
}

pub fn verify(config: &DeviceConfiguration, port: String, baud_rate: u32, timeout_ms: u64) -> Result<VerificationResult, String> {
    let expected = config.assignments.len();
    if expected == 0 { return Err("No sensor assignments exist to verify.".into()); }
    let serial = serialport::new(&port, baud_rate).timeout(Duration::from_millis(250)).open().map_err(|e| format!("Could not open Arduino after upload: {e}"))?;
    let start = Instant::now();
    let mut reader = BufReader::new(serial);
    let mut line = String::new();
    let mut ready_seen = false;
    let mut readings_seen = 0usize;
    let mut valid_readings = 0usize;
    while start.elapsed() < Duration::from_millis(timeout_ms) {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => continue,
            Ok(_) => {
                let text = line.trim();
                if text == "MAINTAIN_AI_SENSOR_NODE_READY" { ready_seen = true; continue; }
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
                    if v.get("reading_type").and_then(|x| x.as_str()) == Some("sensor") && v.get("value").and_then(|x| x.as_f64()).is_some() {
                        readings_seen += 1;
                        valid_readings += 1;
                    }
                }
                if ready_seen && valid_readings >= expected { break; }
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => return Err(format!("Serial verification failed: {e}")),
        }
    }
    let success = ready_seen && valid_readings >= expected;
    Ok(VerificationResult {
        success,
        port,
        ready_seen,
        readings_seen,
        expected_readings: expected,
        valid_readings,
        message: if success {
            "Arduino firmware is responding and producing the expected telemetry. Device is ready for MAINTAIN-AI-IoT-Gateway.".into()
        } else if !ready_seen {
            "Arduino did not report the DeviceOS ready marker before the verification timeout.".into()
        } else {
            format!("Arduino started, but only {valid_readings} of {expected} expected sensor readings were received.")
        },
    })
}
