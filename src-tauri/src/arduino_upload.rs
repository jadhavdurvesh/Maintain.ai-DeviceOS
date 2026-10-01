use serde::{Deserialize, Serialize};
use std::process::Command;
use crate::arduino_toolchain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedBoard { pub address: String, pub port_type: String, pub protocol: String, pub board_name: Option<String>, pub fqbn: Option<String>, pub serial_number: Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResult { pub success: bool, pub port: String, pub fqbn: String, pub stdout: String, pub stderr: String, pub message: String }

fn cli() -> Result<String, String> { arduino_toolchain::status().executable.ok_or_else(|| "Arduino CLI was not found. Install Arduino CLI and restart DeviceOS.".into()) }

pub fn detect() -> Result<Vec<DetectedBoard>, String> {
    let cli = cli()?;
    let out = Command::new(cli).args(["board", "list"]).output().map_err(|e| format!("Could not run Arduino CLI: {e}"))?;
    if !out.status.success() { return Err(String::from_utf8_lossy(&out.stderr).to_string()); }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut boards = Vec::new();
    for line in text.lines().skip(1) {
        let line=line.trim(); if line.is_empty() || line.starts_with("Unknown") { continue; }
        let mut parts=line.split_whitespace();
        let address=match parts.next(){Some(v)=>v.to_string(),None=>continue};
        let protocol=parts.next().unwrap_or("serial").to_string();
        let board_name=line.split("Arduino").nth(1).map(|s|format!("Arduino{}",s.trim()));
        let fqbn=line.split("arduino:").find_map(|s|s.split_whitespace().next()).map(|s|format!("arduino:{s}"));
        boards.push(DetectedBoard{address,port_type:"serial".into(),protocol,board_name,fqbn,serial_number:None});
    }
    Ok(boards)
}

pub fn upload(port:String, fqbn:String, build_dir:String) -> Result<UploadResult,String> {
    let cli=cli()?;
    let out=Command::new(&cli).args(["upload","-p",port.as_str(),"--fqbn",fqbn.as_str(),"--input-dir",build_dir.as_str()]).output().map_err(|e|format!("Could not run Arduino CLI upload: {e}"))?;
    let stdout=String::from_utf8_lossy(&out.stdout).into();
    let stderr=String::from_utf8_lossy(&out.stderr).into();
    Ok(UploadResult{success:out.status.success(),port,fqbn,stdout,stderr,message:if out.status.success(){"Firmware uploaded successfully. Reconnecting to verify the device.".into()}else{"Firmware upload failed. Review the compiler/upload output for details.".into()}})
}
