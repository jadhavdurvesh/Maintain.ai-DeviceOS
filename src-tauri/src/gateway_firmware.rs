use crate::config_store::DeviceConfiguration;
use crate::registry::sensors;
use crate::runtime_firmware::RuntimeFirmware;
use std::collections::HashSet;

pub type GatewayFirmware = RuntimeFirmware;

fn identifier(value: &str) -> String { value.chars().map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_uppercase() } else { '_' }).collect() }
fn escape(value: &str) -> String { value.replace('\\', "\\\\").replace('"', "\\\"") }
fn arduino_pin(board_id: &str, pin: u8, pin_type: &str) -> String { if pin_type == "analog" { if board_id == "esp32-devkit-v1" { pin.to_string() } else { format!("A{}", pin.saturating_sub(14)) } } else { pin.to_string() } }

pub fn generate(config: &DeviceConfiguration) -> Result<GatewayFirmware, String> {
    let board = crate::registry::boards().into_iter().find(|b| b.id == config.board_id)
        .ok_or_else(|| format!("Unknown board: {}", config.board_id))?;

    let mut declarations = String::new();
    let mut setup = String::new();
    let mut reading_objects = Vec::new();
    let mut dht_sensors = HashSet::new();

    for assignment in &config.assignments {
        let sensor = sensors().into_iter().find(|s| s.id == assignment.sensor_id)
            .ok_or_else(|| format!("Unknown sensor: {}", assignment.sensor_id))?;
        let parameter = sensor.parameters.iter().find(|p| p.id == assignment.parameter_id)
            .ok_or_else(|| format!("Unknown parameter: {}", assignment.parameter_id))?;
        let key = identifier(&format!("{}_{}", assignment.sensor_id, assignment.parameter_id));
        let pin = arduino_pin(board.id, assignment.pin, &assignment.pin_type);

        if sensor.id == "dht11" {
            if dht_sensors.insert(sensor.id) {
                declarations.push_str("const uint8_t PIN_DHT11_DATA = ");
                declarations.push_str(&pin);
                declarations.push_str(";\nfloat dht11_temperature=NAN; float dht11_humidity=NAN;\n");
            }
            continue;
        }

        declarations.push_str(&format!("const uint8_t PIN_{} = {};\n", key, pin));
        setup.push_str(&format!("  pinMode(PIN_{}, INPUT);\n", key));
        let expr = if assignment.pin_type == "analog" {
            format!("analogRead(PIN_{})", key)
        } else {
            format!("digitalRead(PIN_{})", key)
        };
        reading_objects.push(format!(
            "    emitReading(\"{}\", \"{}\", (double)({}), \"{}\");",
            escape(sensor.id), escape(parameter.id), expr, escape(parameter.unit)
        ));
    }

    let dht_code = if dht_sensors.is_empty() {
        String::new()
    } else {
        r#"
bool readDht11(uint8_t pin, float &temperature, float &humidity) {
  uint8_t data[5]={0,0,0,0,0};
  pinMode(pin,OUTPUT); digitalWrite(pin,LOW); delay(18);
  digitalWrite(pin,HIGH); delayMicroseconds(30); pinMode(pin,INPUT_PULLUP);
  uint32_t start=micros(); while(digitalRead(pin)==HIGH){if(micros()-start>100)return false;}
  start=micros(); while(digitalRead(pin)==LOW){if(micros()-start>100)return false;}
  start=micros(); while(digitalRead(pin)==HIGH){if(micros()-start>100)return false;}
  for(uint8_t i=0;i<40;i++){
    start=micros(); while(digitalRead(pin)==LOW){if(micros()-start>100)return false;}
    uint32_t highStart=micros();
    while(digitalRead(pin)==HIGH){if(micros()-highStart>100)return false;}
    uint32_t highDuration=micros()-highStart;
    if(highDuration>40)data[i/8]|=(1<<(7-(i%8)));
  }
  if((uint8_t)(data[0]+data[1]+data[2]+data[3])!=data[4])return false;
  humidity=data[0]+data[1]*0.1f; temperature=data[2]+data[3]*0.1f; return true;
}
"#.to_string()
    };

    let mut dht_reads = Vec::new();
    if !dht_sensors.is_empty() {
        for assignment in &config.assignments {
            if assignment.sensor_id == "dht11" {
                let parameter = sensors().into_iter()
                    .find(|s| s.id == "dht11")
                    .and_then(|s| s.parameters.iter().find(|p| p.id == assignment.parameter_id))
                    .ok_or_else(|| "Unknown DHT11 parameter".to_string())?;
                let value = if assignment.parameter_id == "temperature" {
                    "dht11_temperature"
                } else {
                    "dht11_humidity"
                };
                dht_reads.push(format!(
                    "    emitReading(\"dht11\", \"{}\", (double)({}), \"{}\");",
                    escape(parameter.id), value, escape(parameter.unit)
                ));
            }
        }
    }

    let readings_body = if reading_objects.is_empty() && dht_reads.is_empty() {
        String::new()
    } else {
        let mut lines = reading_objects;
        if !dht_reads.is_empty() {
            lines.push("    if (dht11_ok) {".into());
            lines.extend(dht_reads.into_iter().map(|line| format!("  {}", line)));
            lines.push("    }".into());
        }
        lines.join("\n")
    };

    let device = match config.device_id.as_deref() {
        Some(id) if !id.is_empty() => format!("\"{}\"", escape(id)),
        _ => "nullptr".into(),
    };

    let is_esp = board.id == "esp32-devkit-v1";
    let dht_declaration = if dht_sensors.is_empty() { String::new() } else { "bool dht11_ok=false;\n".into() };
    let source = if is_esp {
        let ssid = config.wifi_ssid.as_deref().unwrap_or("").trim();
        let password = config.wifi_password.as_deref().unwrap_or("");
        let api = config.api_base_url.as_deref().unwrap_or("").trim().trim_end_matches('/');
        let key = config.device_key.as_deref().unwrap_or("").trim();
        if ssid.is_empty() || password.is_empty() || api.is_empty() || key.is_empty() {
            return Err("ESP32 requires Wi-Fi SSID, Wi-Fi password, Maintain.ai API URL, and device key before building.".into());
        }
        format!(r###"// Generated by Maintain.ai DeviceOS.
// ESP32 direct-cloud protocol: Wi-Fi -> Maintain.ai /api/devices/ingest.
#include <Arduino.h>
#include <WiFi.h>
#include <HTTPClient.h>
#include <WiFiClientSecure.h>
#include <math.h>

const char* WIFI_SSID="{ssid}";
const char* WIFI_PASSWORD="{password}";
const char* API_BASE_URL="{api}";
const char* DEVICE_KEY="{key}";
const char* DEVICE_ID={device};

{declarations}
{dht_declaration}uint32_t sequenceNumber=0;
uint8_t emittedReadings=0;
WiFiClient plainClient;
WiFiClientSecure secureClient;

void connectWiFi() {{
  if(WiFi.status()==WL_CONNECTED) return;
  WiFi.mode(WIFI_STA);
  WiFi.begin(WIFI_SSID,WIFI_PASSWORD);
  unsigned long started=millis();
  while(WiFi.status()!=WL_CONNECTED && millis()-started<20000) delay(250);
}}

bool sendReadingCloud(const char* readingType,double value,const char* unit) {{
  if(WiFi.status()!=WL_CONNECTED || DEVICE_KEY[0]=='\\0') return false;
  String url=String(API_BASE_URL)+"/api/devices/ingest";
  HTTPClient http;
  bool started=false;
  if(url.startsWith("https://")) {{ secureClient.setInsecure(); started=http.begin(secureClient,url); }}
  else {{ started=http.begin(plainClient,url); }}
  if(!started) return false;
  http.addHeader("Content-Type","application/json");
  http.addHeader("X-Device-Key",DEVICE_KEY);
  String body=String("{{\\"reading_type\\":\\"")+readingType+"\\",\\"value\\":"+String(value,6)+",\\"unit\\":\\""+unit+"\\"}}";
  int code=http.POST(body);
  http.end();
  return code>=200 && code<300;
}}

void emitReading(const char* sensorId,const char* parameterId,double value,const char* unit) {{
  if(emittedReadings>0) Serial.print(",");
  Serial.print("{{\\"sensor_id\\":\\"");
  Serial.print(sensorId);
  Serial.print("\\",\\"parameter_id\\":\\"");
  Serial.print(parameterId);
  Serial.print("\\",\\"value\\":");
  Serial.print(value,6);
  Serial.print(",\\"unit\\":\\"");
  Serial.print(unit);
  Serial.print("\\",\\"timestamp_ms\\":");
  Serial.print(millis());
  Serial.print("}}");
  emittedReadings++;
  sendReadingCloud(parameterId,value,unit);
}}

void emitFrame() {{
  emittedReadings=0;
  {dht_call}
  Serial.print("{{\\"protocol\\":\\"maintain-ai-telemetry\\",\\"protocol_version\\":\\"1.0\\",\\"device_id\\":");
  if(DEVICE_ID==nullptr) Serial.print("null"); else {{ Serial.print("\\""); Serial.print(DEVICE_ID); Serial.print("\\""); }}
  Serial.print(",\\"configuration_id\\":\\"{config}\\",\\"sequence\\":");
  Serial.print(sequenceNumber++);
  Serial.print(",\\"readings\\":[");
{readings_body}
  Serial.println("]}}");
}}

{dht_code}
void setup() {{
  Serial.begin(115200);
{setup}  delay(100);
  Serial.println("MAINTAIN_AI_SENSOR_NODE_READY");
  connectWiFi();
}}

void loop() {{
  connectWiFi();
  emitFrame();
  delay(5000);
}}
"###, ssid=escape(ssid), password=escape(password), api=escape(api), key=escape(key), device=device, declarations=declarations, config=escape(&config.id), readings_body=readings_body, dht_code=dht_code, dht_declaration=dht_declaration, dht_call=if dht_sensors.is_empty() { "" } else { "dht11_ok = readDht11(PIN_DHT11_DATA, dht11_temperature, dht11_humidity);" }, setup=setup)
    } else {
        format!(r#"// Generated by Maintain.ai DeviceOS.
// Gateway protocol: maintain-ai-telemetry v1.0; one envelope per line.
#include <Arduino.h>
#include <math.h>

{declarations}
const char* DEVICE_ID={device};
uint32_t sequenceNumber=0;
uint8_t emittedReadings=0;

void emitReading(const char* sensorId,const char* parameterId,double value,const char* unit) {{
  if(emittedReadings>0) Serial.print(",");
  Serial.print("{{\"sensor_id\":\"");
  Serial.print(sensorId);
  Serial.print("\",\"parameter_id\":\"");
  Serial.print(parameterId);
  Serial.print("\",\"value\":");
  Serial.print(value,6);
  Serial.print(",\"unit\":\"");
  Serial.print(unit);
  Serial.print("\",\"timestamp_ms\":");
  Serial.print(millis());
  Serial.print("}}");
  emittedReadings++;
}}

void emitFrame() {{
  emittedReadings=0;
  Serial.print("{{\"protocol\":\"maintain-ai-telemetry\",\"protocol_version\":\"1.0\",\"device_id\":");
  if(DEVICE_ID==nullptr) Serial.print("null"); else {{Serial.print("\"");Serial.print(DEVICE_ID);Serial.print("\"");}}
  Serial.print(",\"configuration_id\":\"{config}\",\"sequence\":");
  Serial.print(sequenceNumber++);
  Serial.print(",\"readings\":[");
{readings_body}
  Serial.println("]}}");
}}

{dht_code}
void setup() {{
  Serial.begin(115200);
{setup}  delay(100);
  Serial.println("MAINTAIN_AI_SENSOR_NODE_READY");
}}

void loop() {{
  emitFrame();
  delay(1000);
}}
"#, declarations=declarations, device=device, config=escape(&config.id), readings_body=readings_body, dht_code=dht_code, dht_declaration=dht_declaration, dht_call=if dht_sensors.is_empty() { "" } else { "dht11_ok = readDht11(PIN_DHT11_DATA, dht11_temperature, dht11_humidity);" }, setup=setup)
    };

    Ok(RuntimeFirmware {
        configuration_id: config.id.clone(),
        file_name: format!("maintain_ai_gateway_{}.ino", config.id),
        fqbn: board.fqbn.to_string(),
        source,
    })
}
