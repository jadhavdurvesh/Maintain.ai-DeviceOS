# Sensor, Parameter and Machine Signal Model

This document replaces the bootstrap assumption that DeviceOS is a flat list of sensors.

## Core distinction

- **Sensor**: the physical instrument/device.
- **Parameter**: a measurable output exposed by that sensor.
- **Machine signal**: the semantic signal expected by a Maintain.ai machine profile.
- **Reading**: a timestamped value emitted for a parameter/signal.
- **Mapping**: the explicit relationship between a physical parameter and a machine signal.

```text
Machine
  └── Machine signals
          ▲
          │ explicit mapping
          │
      Sensor instance
          └── Parameters[]
                  └── Readings[]
```

## Sensor definition

A sensor definition contains:

- identity
- category
- protocol
- parameters[]
- pin requirements
- supported buses
- board compatibility
- wiring metadata
- firmware module
- calibration metadata
- diagnostics

## Parameter definition

Every parameter has:

- id
- name
- signal_type
- unit
- data_type
- optional minimum/maximum
- calibration information when applicable
- sampling configuration when applicable

A sensor can expose multiple parameters. DHT11 is therefore modeled as one sensor with temperature and humidity parameters, not as two sensors.

## Machine signal vocabulary

The initial semantic vocabulary follows the Maintain.ai machine/telemetry model:

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

The registry may contain more physical sensors than this vocabulary. A physical parameter that does not map to a Maintain.ai signal remains valid DeviceOS data and must not be silently discarded.

## Examples

### DHT11

```text
DHT11
├── temperature → °C
└── humidity    → %
```

### Industrial motor instrumentation

```text
Machine: induction motor
├── current sensor → current
├── voltage sensor → voltage
├── accelerometer/vibration sensor → vibration
├── temperature sensor → temperature
└── tachometer → RPM
```

### Pump instrumentation

```text
Machine: pump
├── pressure sensor → pressure
├── flow sensor → flow
├── current sensor → current
├── vibration sensor → vibration
└── temperature sensor → temperature
```

## Important rule

DeviceOS must never infer that a particular physical sensor is the only possible source of a machine signal. Multiple sensor technologies may provide the same semantic signal, and one sensor may provide multiple signals.

## Maintain.ai boundary

DeviceOS is responsible for physical acquisition and configuration. Maintain.ai 3 remains the system of record for machine state, alerts, maintenance, work orders, history and ML outcomes.
