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
- voltage
- reserved pins
- upload capabilities

Initial board definitions:
- arduino-uno-r3
- arduino-nano

Planned:
- arduino-mega-2560
- esp32-devkit
- esp32-wroom
- esp8266-nodemcu

## Sensor Registry

Each sensor definition declares:
- id
- name
- protocol
- parameters
- pin capabilities
- power range
- library requirements
- compatible boards
- firmware module
- wiring map
- calibration support
- diagnostic metadata

Initial:
- dht11
- sw420
- generic-analog-input
- generic-digital-input

Planned:
- dht22
- ds18b20
- bmp280
- bme280
- mpu6050
- hc-sr04
- acs712
- LDR
- industrial accelerometers
- 4-20mA
- Modbus/RS485

## Versioning

Use schema_version and definition_version. Changes to compatibility, pin maps, libraries or safety metadata require a definition version bump and regression coverage.
