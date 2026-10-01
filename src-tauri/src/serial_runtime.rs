use serde::{Deserialize, Serialize};
use serialport::{SerialPort, SerialPortInfo};
use std::io::{Read, Write};
use std::time::Duration;

use crate::telemetry::{TelemetryEnvelope, TelemetryReading};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialRuntimeConfig {
    pub port: String,
    pub baud_rate: u32,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFrame {
    pub protocol: String,
    pub protocol_version: String,
    pub device_id: Option<String>,
    pub configuration_id: String,
    pub sequence: u64,
    pub readings: Vec<TelemetryReading>,
}

pub fn list_ports() -> Result<Vec<SerialPortInfo>, String> {
    serialport::available_ports().map_err(|e| e.to_string())
}

pub fn open(config: &SerialRuntimeConfig) -> Result<Box<dyn SerialPort>, String> {
    serialport::new(&config.port, config.baud_rate)
        .timeout(Duration::from_millis(config.timeout_ms))
        .open()
        .map_err(|e| e.to_string())
}

pub fn send_envelope(port: &mut dyn SerialPort, envelope: &TelemetryEnvelope) -> Result<(), String> {
    let payload = serde_json::to_vec(envelope).map_err(|e| e.to_string())?;
    port.write_all(&payload).map_err(|e| e.to_string())?;
    port.write_all(b"\n").map_err(|e| e.to_string())?;
    port.flush().map_err(|e| e.to_string())
}

pub fn read_frame(port: &mut dyn SerialPort) -> Result<DeviceFrame, String> {
    let mut buffer = Vec::with_capacity(4096);
    let mut byte = [0u8; 1];
    loop {
        match port.read(&mut byte) {
            Ok(1) if byte[0] == b'\n' => break,
            Ok(1) => {
                buffer.push(byte[0]);
                if buffer.len() > 64 * 1024 { return Err("Serial frame exceeds 64 KiB".into()); }
            }
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        }
    }
    serde_json::from_slice(&buffer).map_err(|e| format!("Invalid DeviceOS telemetry frame: {e}"))
}
