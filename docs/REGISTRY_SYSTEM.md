# Registry System

Registries are versioned data contracts and must remain independent from the UI.

## Board Registry

Each board definition declares:
- id
- name
- platform
- FQBN
- architecture/MCU
- digital pins
- analog pins
- PWM pins
- UART
- I2C
- SPI
- supported buses
- voltage
- reserved pins
- upload capabilities

Initial boards:
- Arduino Uno R3
- Arduino Nano

Planned boards:
- Arduino Mega 2560
- ESP32 DevKit
- ESP32 WROOM
- ESP8266 NodeMCU

## Sensor Registry

A sensor is a physical instrument, not a machine signal.

Each sensor definition declares:
- id
- name
- category
- protocol
- parameters[]
- pin requirements
- supported buses
- power range
- library requirements
- compatible boards
- firmware module
- wiring map
- calibration support
- diagnostic metadata

A sensor may expose multiple parameters. For example, DHT11 exposes temperature and humidity.

## Parameter Registry

Every parameter declares:
- id
- name
- signal_type
- unit
- data_type
- optional min/max
- calibration metadata
- sampling metadata

Parameters represent what the physical sensor can measure.

## Machine Signal Registry

Machine signals represent semantic values used by Maintain.ai machine profiles. Initial vocabulary:
- temperature
- humidity
- vibration
- current
- voltage
- pressure
- flow
- speed
- load
- RPM
- distance

A machine signal can have multiple possible physical sensor sources.

## Mapping

Configuration stores an explicit mapping:

```text
sensor instance → parameter → machine signal
```

DeviceOS must never assume that one sensor type uniquely defines a machine signal.

## Physical sensor expansion

The registry is designed to grow beyond the current implementation. Planned examples include:
- DHT22
- DS18B20
- BMP280
- BME280
- MPU6050
- HC-SR04
- ACS712
- LDR
- industrial accelerometers
- 4-20mA sensors
- Modbus/RS485 instruments

These are registry entries, not the definition of the entire Maintain.ai signal model.

## Versioning

Use schema_version and definition_version. Changes to compatibility, parameters, pin maps, libraries, wiring or safety metadata require a definition version bump and regression coverage.
