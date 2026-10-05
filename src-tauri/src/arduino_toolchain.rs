use serde::{Deserialize, Serialize};
use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};
use crate::{config_store::DeviceConfiguration, gateway_firmware};

const CLI_VERSION: &str = "1.5.1";
const WINDOWS_CLI_URL: &str = "https://downloads.arduino.cc/arduino-cli/arduino-cli_1.5.1_Windows_64bit.zip";
const WINDOWS_CLI_SHA256: &str = "FABE42E0EB04D00E776A66178299FF95A46C623DBC260F997E58FD514853DD40";
const ESP32_CORE_INDEX: &str = "https://raw.githubusercontent.com/espressif/arduino-esp32/gh-pages/package_esp32_index.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainStatus { pub installed: bool, pub executable: Option<String>, pub version: Option<String>, pub message: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult { pub success: bool, pub output_dir: String, pub source_file: String, pub saved_dir: Option<String>, pub stdout: String, pub stderr: String, pub fqbn: String }

fn command_works(path: &str) -> bool { Command::new(path).arg("version").output().map(|o| o.status.success()).unwrap_or(false) }

fn managed_cli_path() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(root) = std::env::var("LOCALAPPDATA") {
            let p = std::path::PathBuf::from(root).join("Maintain.ai").join("DeviceOS").join("toolchain").join("arduino-cli.exe");
            if p.is_file() { return Some(p.to_string_lossy().into()); }
        }
    }
    None
}

fn find_cli() -> Option<String> {
    if let Some(p) = managed_cli_path() { if command_works(&p) { return Some(p); } }
    let mut candidates: Vec<std::path::PathBuf> = Vec::new();
    #[cfg(target_os = "windows")]
    {
        candidates.push(std::path::PathBuf::from("arduino-cli.exe"));
        candidates.push(std::path::PathBuf::from("arduino-cli"));
        if let Ok(root) = std::env::var("PROGRAMFILES") {
            candidates.push(std::path::PathBuf::from(root).join("Arduino CLI").join("arduino-cli.exe"));
        }
        if let Ok(root) = std::env::var("USERPROFILE") {
            candidates.push(std::path::PathBuf::from(&root).join("scoop").join("shims").join("arduino-cli.exe"));
            candidates.push(std::path::PathBuf::from(&root).join("bin").join("arduino-cli.exe"));
        }
        candidates.push(std::path::PathBuf::from(r"C:\ProgramData\chocolatey\bin\arduino-cli.exe"));
    }
    #[cfg(not(target_os = "windows"))]
    {
        candidates.push(std::path::PathBuf::from("arduino-cli"));
        candidates.push(std::path::PathBuf::from("/usr/local/bin/arduino-cli"));
        candidates.push(std::path::PathBuf::from("/usr/bin/arduino-cli"));
        candidates.push(std::path::PathBuf::from("/opt/homebrew/bin/arduino-cli"));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() { candidates.push(dir.join(if cfg!(target_os = "windows") { "arduino-cli.exe" } else { "arduino-cli" })); }
    }
    candidates.into_iter().find_map(|p| {
        let s=p.to_string_lossy().into_owned();
        command_works(&s).then_some(s)
    })
}

pub fn status() -> ToolchainStatus {
    match find_cli() {
        Some(cli) => match Command::new(&cli).arg("version").output() {
            Ok(o) if o.status.success() => ToolchainStatus { installed: true, executable: Some(cli), version: Some(String::from_utf8_lossy(&o.stdout).trim().to_string()), message: "Arduino CLI is ready.".into() },
            Ok(o) => ToolchainStatus { installed: false, executable: None, version: None, message: String::from_utf8_lossy(&o.stderr).trim().to_string() },
            Err(e) => ToolchainStatus { installed: false, executable: None, version: None, message: e.to_string() }
        },
        None => ToolchainStatus { installed: false, executable: None, version: None, message: "Arduino CLI is not installed yet. DeviceOS can install its managed toolchain automatically.".into() }
    }
}

pub fn ensure_cli() -> Result<String, String> {
    if let Some(cli)=find_cli() { return Ok(cli); }
    #[cfg(target_os = "windows")]
    {
        let root=std::env::var("LOCALAPPDATA").map(std::path::PathBuf::from)
            .map(|p| p.join("Maintain.ai").join("DeviceOS").join("toolchain"))
            .map_err(|_| "LOCALAPPDATA is unavailable; DeviceOS cannot choose a managed toolchain location.".to_string())?;
        fs::create_dir_all(&root).map_err(|e|e.to_string())?;
        let archive=root.join("arduino-cli.zip");
        let exe=root.join("arduino-cli.exe");
        let script=format!(
            "$ErrorActionPreference='Stop'; Invoke-WebRequest -UseBasicParsing -Uri '{}' -OutFile '{}'; $h=(Get-FileHash -Algorithm SHA256 -Path '{}').Hash; if ($h -ne '{}') {{ Remove-Item -Force '{}'; throw 'Arduino CLI checksum verification failed.' }}; Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force; if (!(Test-Path '{}')) {{ throw 'Arduino CLI executable was not found after extraction.' }}; Remove-Item -Force '{}'",
            WINDOWS_CLI_URL, archive.display(), archive.display(), WINDOWS_CLI_SHA256, archive.display(), archive.display(), root.display(), exe.display(), archive.display()
        );
        let out=Command::new("powershell.exe").args(["-NoProfile","-NonInteractive","-ExecutionPolicy","Bypass","-Command",&script]).output().map_err(|e|format!("Could not install Arduino CLI: {e}"))?;
        if !out.status.success() {
            return Err(format!("Arduino CLI installation failed: {}", String::from_utf8_lossy(&out.stderr).trim()));
        }
        if !command_works(&exe.to_string_lossy()) { return Err("Arduino CLI was downloaded but could not be started.".into()); }
        return Ok(exe.to_string_lossy().into());
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("Arduino CLI is not installed. Install the official Arduino CLI and run Scan USB again.".into())
    }
}

fn stamp() -> String { SystemTime::now().duration_since(UNIX_EPOCH).map(|d|d.as_millis().to_string()).unwrap_or_else(|_| "0".into()) }

pub fn compile(config: &DeviceConfiguration) -> Result<BuildResult, String> {
    let fw = gateway_firmware::generate(config)?;
    compile_source(config, &fw.source, &fw.fqbn)
}

pub fn compile_source(config: &DeviceConfiguration, source: &str, fqbn: &str) -> Result<BuildResult, String> {
    let cli = ensure_cli()?;
    let root = std::env::temp_dir().join(format!("maintain-ai-deviceos-{}", stamp()));
    let sketch_dir = root.join("gateway_firmware");
    fs::create_dir_all(&sketch_dir).map_err(|e| e.to_string())?;
    let source_file = sketch_dir.join("gateway_firmware.ino");
    fs::write(&source_file, source).map_err(|e| e.to_string())?;
    let output_dir = root.join("build");
    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;

    let core_id = if fqbn.starts_with("esp32:") { "esp32:esp32" } else { "arduino:avr" };
    if core_id == "esp32:esp32" {
        let index = Command::new(&cli).args(["core","update-index","--additional-urls",ESP32_CORE_INDEX]).output()
            .map_err(|e| format!("Could not update ESP32 board index: {e}"))?;
        if !index.status.success() {
            return Err(format!("ESP32 board index could not be updated. {}", String::from_utf8_lossy(&index.stderr).trim()));
        }
        let core = Command::new(&cli).args(["core","install",core_id,"--additional-urls",ESP32_CORE_INDEX]).output()
            .map_err(|e|format!("Could not prepare ESP32 board support: {e}"))?;
        if !core.status.success() {
            return Err(format!("ESP32 board support could not be installed. {}", String::from_utf8_lossy(&core.stderr).trim()));
        }
    } else {
        let core = Command::new(&cli).args(["core","install",core_id]).output()
            .map_err(|e|format!("Could not prepare Arduino AVR board support: {e}"))?;
        if !core.status.success() {
            return Err(format!("Arduino AVR board support could not be installed. {}", String::from_utf8_lossy(&core.stderr).trim()));
        }
    }

    let output = Command::new(&cli)
        .args(["compile", "--fqbn", fqbn, "--output-dir", output_dir.to_string_lossy().as_ref(), sketch_dir.to_string_lossy().as_ref()])
        .output().map_err(|e| format!("Could not run Arduino CLI: {e}"))?;
    Ok(BuildResult { success: output.status.success(), output_dir: output_dir.to_string_lossy().into(), source_file: source_file.to_string_lossy().into(), saved_dir: save_firmware_copy(config, &source_file, &output_dir).ok(), stdout: String::from_utf8_lossy(&output.stdout).into(), stderr: String::from_utf8_lossy(&output.stderr).into(), fqbn: fqbn.to_string() })
}

fn documents_dir() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    if let Ok(user) = std::env::var("USERPROFILE") { return Some(std::path::PathBuf::from(user).join("Documents")); }
    #[cfg(not(target_os = "windows"))]
    if let Ok(home) = std::env::var("HOME") { return Some(std::path::PathBuf::from(home).join("Documents")); }
    None
}

fn save_firmware_copy(config: &DeviceConfiguration, source_file: &std::path::Path, output_dir: &std::path::Path) -> Result<String, String> {
    let root = documents_dir().ok_or("Documents folder could not be located")?
        .join("Maintain.ai").join("DeviceOS").join("Firmware")
        .join(&config.id);
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let source_target = root.join("gateway_firmware.ino");
    fs::copy(source_file, &source_target).map_err(|e| e.to_string())?;
    let build_target = root.join("build");
    if build_target.exists() { fs::remove_dir_all(&build_target).map_err(|e| e.to_string())?; }
    fs::create_dir_all(&build_target).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(output_dir).map_err(|e| e.to_string())? {
        let entry=entry.map_err(|e| e.to_string())?;
        if entry.file_type().map_err(|e| e.to_string())?.is_file() {
            fs::copy(entry.path(), build_target.join(entry.file_name())).map_err(|e| e.to_string())?;
        }
    }
    Ok(root.to_string_lossy().into())
}
