use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
use crate::{config_store::DeviceConfiguration, gateway_firmware};

const ARDUINO_CLI_VERSION: &str = "1.5.1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainStatus { pub installed: bool, pub executable: Option<String>, pub version: Option<String>, pub message: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult { pub success: bool, pub output_dir: String, pub source_file: String, pub stdout: String, pub stderr: String, pub fqbn: String }

fn toolchain_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("Maintain.ai")
        .join("DeviceOS")
        .join("toolchain")
}

fn local_cli_path() -> PathBuf {
    if cfg!(target_os = "windows") { toolchain_dir().join("arduino-cli.exe") }
    else { toolchain_dir().join("arduino-cli") }
}

fn find_cli() -> Option<String> {
    let local = local_cli_path();
    if local.is_file() {
        return Some(local.to_string_lossy().into());
    }
    let candidates = if cfg!(target_os = "windows") {
        vec!["arduino-cli.exe", "arduino-cli"]
    } else {
        vec!["arduino-cli"]
    };
    candidates.into_iter()
        .find(|c| Command::new(c).arg("version").output().map(|o| o.status.success()).unwrap_or(false))
        .map(str::to_string)
}

fn platform_download() -> Result<(&'static str, &'static str), String> {
    if cfg!(target_os = "windows") {
        if cfg!(target_arch = "x86_64") {
            return Ok(("https://downloads.arduino.cc/arduino-cli/arduino-cli_1.5.1_Windows_64bit.zip", "zip"));
        }
        if cfg!(target_arch = "aarch64") {
            return Ok(("https://downloads.arduino.cc/arduino-cli/arduino-cli_1.5.1_Windows_ARM64.zip", "zip"));
        }
        return Err("This Windows architecture is not supported by the automatic Arduino CLI installer.".into());
    }
    if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            return Ok(("https://downloads.arduino.cc/arduino-cli/arduino-cli_1.5.1_macOS_ARM64.tar.gz", "tar.gz"));
        }
        if cfg!(target_arch = "x86_64") {
            return Ok(("https://downloads.arduino.cc/arduino-cli/arduino-cli_1.5.1_macOS_64bit.tar.gz", "tar.gz"));
        }
        return Err("This macOS architecture is not supported by the automatic Arduino CLI installer.".into());
    }
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "aarch64") {
            return Ok(("https://downloads.arduino.cc/arduino-cli/arduino-cli_1.5.1_Linux_ARM64.tar.gz", "tar.gz"));
        }
        if cfg!(target_arch = "arm") {
            return Ok(("https://downloads.arduino.cc/arduino-cli/arduino-cli_1.5.1_Linux_ARMv7.tar.gz", "tar.gz"));
        }
        if cfg!(target_arch = "x86_64") {
            return Ok(("https://downloads.arduino.cc/arduino-cli/arduino-cli_1.5.1_Linux_64bit.tar.gz", "tar.gz"));
        }
        return Err("This Linux architecture is not supported by the automatic Arduino CLI installer.".into());
    }
    Err("This operating system is not supported by the automatic Arduino CLI installer.".into())
}

fn download_file(url: &str, destination: &Path) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        let status = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command"])
            .arg(format!("Invoke-WebRequest -UseBasicParsing -Uri '{}' -OutFile '{}'", url.replace(''', "''"), destination.display().to_string().replace(''', "''")))
            .status()
            .map_err(|e| format!("Could not start PowerShell: {e}"))?;
        if !status.success() { return Err("PowerShell could not download Arduino CLI.".into()); }
    } else {
        let status = Command::new("curl")
            .args(["-fL", "--retry", "3", "-o"])
            .arg(destination)
            .arg(url)
            .status()
            .map_err(|e| format!("Could not start curl: {e}. Install curl or Arduino CLI manually."))?;
        if !status.success() { return Err("curl could not download Arduino CLI.".into()); }
    }
    Ok(())
}

fn install_cli() -> Result<String, String> {
    let (url, archive_type) = platform_download()?;
    let dir = toolchain_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Could not create DeviceOS toolchain directory: {e}"))?;
    let archive = dir.join(if archive_type == "zip" { "arduino-cli.zip" } else { "arduino-cli.tar.gz" });
    download_file(url, &archive)?;

    if archive_type == "zip" {
        let status = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command"])
            .arg(format!("Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force", archive.display(), dir.display()))
            .status()
            .map_err(|e| format!("Could not extract Arduino CLI: {e}"))?;
        if !status.success() { return Err("Could not extract the Arduino CLI archive.".into()); }
    } else {
        let status = Command::new("tar")
            .args(["-xzf"])
            .arg(&archive)
            .args(["-C"])
            .arg(&dir)
            .status()
            .map_err(|e| format!("Could not extract Arduino CLI: {e}"))?;
        if !status.success() { return Err("Could not extract the Arduino CLI archive.".into()); }
    }

    let cli = local_cli_path();
    if !cli.is_file() {
        return Err(format!("Arduino CLI downloaded, but the executable was not found at {}.", cli.display()));
    }
    if !cfg!(target_os = "windows") {
        let _ = Command::new("chmod").args(["+x"]).arg(&cli).status();
    }
    let _ = fs::remove_file(&archive);
    Ok(cli.to_string_lossy().into())
}

fn cli() -> Result<String, String> {
    if let Some(cli) = find_cli() { return Ok(cli); }
    install_cli()
}

fn stamp() -> String {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis().to_string()).unwrap_or_else(|_| "0".into())
}

pub fn status() -> ToolchainStatus {
    match find_cli() {
        Some(cli) => match Command::new(&cli).arg("version").output() {
            Ok(o) if o.status.success() => ToolchainStatus { installed: true, executable: Some(cli), version: Some(String::from_utf8_lossy(&o.stdout).trim().to_string()), message: "Arduino CLI is ready.".into() },
            Ok(o) => ToolchainStatus { installed: false, executable: None, version: None, message: String::from_utf8_lossy(&o.stderr).trim().to_string() },
            Err(e) => ToolchainStatus { installed: false, executable: None, version: None, message: e.to_string() }
        },
        None => ToolchainStatus { installed: false, executable: None, version: format!("Arduino CLI {} will be installed automatically when Arduino detection/build/upload is used.", ARDUINO_CLI_VERSION), message: format!("Arduino CLI is not installed yet. DeviceOS will install the official Arduino CLI {} automatically when you scan, build, or upload.", ARDUINO_CLI_VERSION) }
    }
}

pub fn compile(config: &DeviceConfiguration) -> Result<BuildResult, String> {
    let cli = cli()?;
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
