use serde::{Deserialize, Serialize};
use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};
use crate::{config_store::DeviceConfiguration, gateway_firmware};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainStatus { pub installed: bool, pub executable: Option<String>, pub version: Option<String>, pub message: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult { pub success: bool, pub output_dir: String, pub source_file: String, pub stdout: String, pub stderr: String, pub fqbn: String }

fn find_cli() -> Option<String> {
    let candidates = if cfg!(target_os = "windows") { vec!["arduino-cli.exe", "arduino-cli"] } else { vec!["arduino-cli"] };
    candidates.into_iter().find(|c| Command::new(c).arg("version").output().is_ok()).map(str::to_string)
}

fn stamp() -> String { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis().to_string()).unwrap_or_else(|_| "0".into()) }

pub fn status() -> ToolchainStatus {
    match find_cli() {
        Some(cli) => match Command::new(&cli).arg("version").output() {
            Ok(o) => ToolchainStatus { installed: true, executable: Some(cli), version: Some(String::from_utf8_lossy(&o.stdout).trim().to_string()), message: "Arduino CLI is ready.".into() },
            Err(e) => ToolchainStatus { installed: false, executable: None, version: None, message: e.to_string() }
        },
        None => ToolchainStatus { installed: false, executable: None, version: None, message: "Arduino CLI was not found. Install Arduino CLI and restart DeviceOS.".into() }
    }
}

pub fn compile(config: &DeviceConfiguration) -> Result<BuildResult, String> {
    let cli = find_cli().ok_or_else(|| status().message)?;
    let fw = gateway_firmware::generate(config)?;
    let root = std::env::temp_dir().join(format!("maintain-ai-deviceos-{}", stamp()));
    let sketch_dir = root.join("gateway_firmware");
    fs::create_dir_all(&sketch_dir).map_err(|e| e.to_string())?;
    let source_file = sketch_dir.join("gateway_firmware.ino");
    fs::write(&source_file, fw.source).map_err(|e| e.to_string())?;
    let output_dir = root.join("build");
    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
    let output = Command::new(&cli)
        .args(["compile", "--fqbn", fw.fqbn.as_str(), "--output-dir", output_dir.to_string_lossy().as_ref(), sketch_dir.to_string_lossy().as_ref()])
        .output().map_err(|e| format!("Could not run Arduino CLI: {e}"))?;
    Ok(BuildResult { success: output.status.success(), output_dir: output_dir.to_string_lossy().into(), source_file: source_file.to_string_lossy().into(), stdout: String::from_utf8_lossy(&output.stdout).into(), stderr: String::from_utf8_lossy(&output.stderr).into(), fqbn: fw.fqbn })
}
