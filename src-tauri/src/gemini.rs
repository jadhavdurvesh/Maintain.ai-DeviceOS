use crate::config_store::DeviceConfiguration;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeminiSettings { pub api_key: String, #[serde(default = "default_model")] pub model: String }
fn default_model() -> String { "gemini-3.8-flash".into() }
fn fallback_models(selected:&str)->Vec<String>{ let mut v=vec![selected.to_string(),"gemini-3.7-flash".into(),"gemini-3.6-flash".into(),"gemini-3.5-flash".into(),"gemini-3.1-flash-lite".into()]; v.dedup(); v }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiVerification { pub ok: bool, pub changed: bool, pub summary: String, pub issues: Vec<String>, pub corrected_source: Option<String> }

fn settings_path() -> Result<PathBuf, String> {
    let root = dirs::data_dir().ok_or("Could not locate the local application data folder.")?;
    Ok(root.join("Maintain.ai").join("DeviceOS").join("gemini.json"))
}
pub fn load_settings() -> Result<GeminiSettings, String> {
    let path=settings_path()?; if !path.exists(){return Ok(GeminiSettings{api_key:String::new(),model:default_model()});}
    let text=fs::read_to_string(path).map_err(|e|format!("Could not read Gemini settings: {e}"))?;
    serde_json::from_str(&text).map_err(|e|format!("Could not parse Gemini settings: {e}"))
}
pub fn save_settings(settings: GeminiSettings)->Result<(),String>{
    let path=settings_path()?; if let Some(parent)=path.parent(){fs::create_dir_all(parent).map_err(|e|e.to_string())?;}
    let clean=GeminiSettings{api_key:settings.api_key.trim().to_string(),model:if settings.model.trim().is_empty(){default_model()}else{settings.model.trim().to_string()}};
    fs::write(path,serde_json::to_string_pretty(&clean).map_err(|e|e.to_string())?).map_err(|e|format!("Could not save Gemini settings: {e}"))
}
#[derive(Deserialize)] struct GenerateResponse{candidates:Option<Vec<Candidate>>}
#[derive(Deserialize)] struct Candidate{content:Option<Content>}
#[derive(Deserialize)] struct Content{parts:Option<Vec<Part>>}
#[derive(Deserialize)] struct Part{text:Option<String>}
fn extract_text(r:GenerateResponse)->Result<String,String>{r.candidates.and_then(|mut c|c.drain(..).next()).and_then(|c|c.content).and_then(|c|c.parts).and_then(|mut p|p.drain(..).find_map(|x|x.text)).ok_or("Gemini returned no text.".into())}
pub fn verify(source:&str,config:&DeviceConfiguration)->Result<GeminiVerification,String>{
    let settings=load_settings()?; if settings.api_key.trim().is_empty(){return Err("Gemini API key is not configured. Open Settings and add your key.".into());}
    let model=if settings.model.trim().is_empty(){default_model()}else{settings.model.trim().to_string()};
    let wifi_secret=config.wifi_password.clone().unwrap_or_default();
    let device_secret=config.device_key.clone().unwrap_or_default();
    let mut ai_source=source.to_string();
    if !wifi_secret.is_empty(){ai_source=ai_source.replace(&wifi_secret,"<DEVICE_WIFI_PASSWORD>");}
    if !device_secret.is_empty(){ai_source=ai_source.replace(&device_secret,"<DEVICE_KEY>");}
    let mut ai_config=config.clone(); ai_config.wifi_password=None; ai_config.device_key=None;
    let prompt=format!("You are the firmware verification engineer for Maintain.ai DeviceOS. Review this generated embedded C++ firmware for the exact device configuration. Return ONLY valid JSON with schema {{\"ok\":true/false,\"changed\":true/false,\"summary\":\"short summary\",\"issues\":[\"issue\"],\"corrected_source\":\"complete corrected source or null\"}}. Preserve the Maintain.ai telemetry protocol, configuration identity, and every configured sensor pin. Fix only compile errors, invalid C/C++, invalid JSON generation, obvious sensor read logic bugs, or protocol-breaking mistakes. Do not move sensors, remove required telemetry, or add new secrets. If correct: ok=true, changed=false, corrected_source=null. If correction is needed, corrected_source must be the complete compilable source. machine={} board={} assignments={} firmware={} ",ai_config.machine_type_id,ai_config.board_id,serde_json::to_string(&ai_config.assignments).unwrap_or_default(),ai_source);
    let body=serde_json::json!({"contents":[{"parts":[{"text":prompt}]}],"generationConfig":{"temperature":0.1,"responseMimeType":"application/json"}});
    let client=Client::builder().timeout(std::time::Duration::from_secs(60)).build().map_err(|e|format!("Could not initialize AI client: {e}"))?;
    let mut last_error=String::new();
    for candidate_model in fallback_models(&model) {
        for attempt in 0..2 {
            let url=format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",candidate_model);
            let response=client.post(url).header("x-goog-api-key",settings.api_key.trim()).header("Content-Type","application/json").json(&body).send().map_err(|e|format!("AI request failed: {e}"))?;
            let status=response.status(); let text=response.text().map_err(|e|format!("Could not read AI response: {e}"))?;
            if status.is_success(){
                let parsed:GenerateResponse=serde_json::from_str(&text).map_err(|e|format!("Invalid AI response: {e}"))?; let output=extract_text(parsed)?;
                let clean=output.trim().trim_start_matches("```json").trim_end_matches("```").trim();
                let mut result:GeminiVerification=serde_json::from_str(clean).map_err(|e|format!("AI returned invalid verification JSON: {e}"))?;
                if let Some(corrected)=result.corrected_source.as_mut(){ if !wifi_secret.is_empty(){*corrected=corrected.replace("<DEVICE_WIFI_PASSWORD>",&wifi_secret);} if !device_secret.is_empty(){*corrected=corrected.replace("<DEVICE_KEY>",&device_secret);} }
                return Ok(result)
            }
            last_error=format!("AI model {} returned HTTP {}: {}",candidate_model,status,text);
            if status.as_u16()!=429 && status.as_u16()!=500 && status.as_u16()!=503 { break; }
            if attempt==0 { std::thread::sleep(std::time::Duration::from_secs(2)); }
        }
    }
    Err(last_error) let clean=output.trim().trim_start_matches("```json").trim_end_matches("```").trim();
    let mut result:GeminiVerification=serde_json::from_str(clean).map_err(|e|format!("Gemini returned invalid verification JSON: {e}"))?;
    if let Some(corrected)=result.corrected_source.as_mut(){ if !wifi_secret.is_empty(){*corrected=corrected.replace("<DEVICE_WIFI_PASSWORD>",&wifi_secret);} if !device_secret.is_empty(){*corrected=corrected.replace("<DEVICE_KEY>",&device_secret);} }
    Ok(result)
}