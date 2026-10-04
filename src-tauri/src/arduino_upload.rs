use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::arduino_toolchain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedBoard { pub address: String, pub port_type: String, pub protocol: String, pub board_name: Option<String>, pub fqbn: Option<String>, pub serial_number: Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResult { pub success: bool, pub port: String, pub fqbn: String, pub stdout: String, pub stderr: String, pub message: String }

fn cli() -> Result<String, String> { arduino_toolchain::status().executable.ok_or_else(|| "Arduino CLI was not found. Install Arduino CLI and restart DeviceOS.".into()) }

pub fn detect() -> Result<Vec<DetectedBoard>, String> {
    let cli = arduino_toolchain::ensure_cli()?;
    let out = Command::new(cli).args(["board", "list"]).output().map_err(|e| format!("Could not run Arduino CLI: {e}"))?;
    if !out.status.success() { return Err(String::from_utf8_lossy(&out.stderr).to_string()); }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut boards = Vec::new();
    for line in text.lines().skip(1) {
        let line=line.trim(); if line.is_empty() || line.starts_with("Unknown") { continue; }
        let mut parts=line.split_whitespace();
        let address=match parts.next(){Some(v)=>v.to_string(),None=>continue};
        let protocol=parts.next().unwrap_or("serial").to_string();
        let fqbn=parts.find(|token| { let clean=token.trim_matches(|c:char| c==',' || c==')'); crate::registry::boards().iter().any(|b| b.fqbn==clean) }).map(|token| token.trim_matches(|c:char| c==',' || c==')').to_string());
        let board_name=fqbn.as_ref().and_then(|id| crate::registry::boards().into_iter().find(|b| b.fqbn==id).map(|b| b.name.to_string()));
        boards.push(DetectedBoard{address,port_type:"serial".into(),protocol,board_name,fqbn,serial_number:None});
    }
    Ok(boards)
}

fn verify_after_upload(port: &str) -> Result<(), String> {
    sleep(Duration::from_millis(1500));
    let serial = serialport::new(port, 115200)
        .timeout(Duration::from_millis(250))
        .open()
        .map_err(|e| format!("Firmware uploaded, but DeviceOS could not reconnect to the Arduino: {e}"))?;
    let start = Instant::now();
    let mut reader = BufReader::new(serial);
    let mut line = String::new();
    let mut ready_seen = false;
    let mut telemetry_seen = false;
    while start.elapsed() < Duration::from_secs(10) {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => continue,
            Ok(_) => {
                let text = line.trim();
                if text == "MAINTAIN_AI_SENSOR_NODE_READY" { ready_seen = true; continue; }
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
                    let protocol_ok = value.get("protocol").and_then(|v| v.as_str()) == Some("maintain-ai-telemetry")
                        && value.get("protocol_version").and_then(|v| v.as_str()) == Some("1.0");
                    let readings_ok = value.get("readings").and_then(|v| v.as_array()).map(|items| {
                        items.iter().any(|item| {
                            item.get("sensor_id").and_then(|v| v.as_str()).is_some()
                                && item.get("parameter_id").and_then(|v| v.as_str()).is_some()
                                && item.get("value").and_then(|v| v.as_f64()).is_some()
                        })
                    }).unwrap_or(false);
                    if protocol_ok && readings_ok { telemetry_seen = true; }
                }
                if ready_seen && telemetry_seen { return Ok(()); }
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => return Err(format!("Device verification failed while reading serial data: {e}")),
        }
    }
    if !ready_seen { return Err("Firmware uploaded, but the Arduino did not report the DeviceOS ready marker within 10 seconds.".into()); }
    if !telemetry_seen { return Err("Firmware uploaded and DeviceOS started, but no valid sensor telemetry was received within 10 seconds.".into()); }
    Err("Device verification timed out.".into())
}

pub fn upload(port:String, fqbn:String, build_dir:String) -> Result<UploadResult,String> {
    let cli=cli()?;
    let out=Command::new(&cli).args(["upload","-p",port.as_str(),"--fqbn",fqbn.as_str(),"--input-dir",build_dir.as_str()]).output().map_err(|e|format!("Could not run Arduino CLI upload: {e}"))?;
    let stdout=String::from_utf8_lossy(&out.stdout).into();
    let stderr=String::from_utf8_lossy(&out.stderr).into();
    if !out.status.success() {
        return Ok(UploadResult{success:false,port,fqbn,stdout,stderr,message:"Firmware upload failed. Review the compiler/upload output for details.".into()});
    }
    match verify_after_upload(&port) {
        Ok(()) => Ok(UploadResult{success:true,port,fqbn:fqbn.clone(),stdout,stderr,message:if fqbn.starts_with("esp32:") { "Firmware uploaded and verified. ESP32 is ready for direct Wi-Fi telemetry to Maintain.ai.".into() } else { "Firmware uploaded and verified. Controller is ready for MAINTAIN-AI-IoT-Gateway.".into() }}),
        Err(message) => Ok(UploadResult{success:false,port,fqbn,stdout,stderr,message}),
    }
}
