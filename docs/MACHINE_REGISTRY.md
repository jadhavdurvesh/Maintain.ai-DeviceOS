# Machine Registry

The machine registry is the DeviceOS bridge between a physical installation and Maintain.ai machine semantics.

## Structure

```text
Machine Type
├── Components
│   └── supported signal IDs
└── supported signal IDs
```

## Initial machine types

The implementation starts with the core Maintain.ai machine categories represented in the current project model:

- Induction Motor
- Pump
- Conveyor
- Compressor
- CNC Machine
- Generator
- Transformer
- Fan / Blower

This is an extensible registry, not a claim that these are the only machine types in Maintain.ai 3.

## Configuration mapping

```text
Machine Type
    ↓
Component
    ↓
Machine Signal
    ↑
Sensor Parameter
    ↑
Sensor Instance
```

A configuration must retain this mapping explicitly. The UI should not silently convert a physical parameter into a machine signal merely because the names happen to match.

## Compatibility

A machine type defines which signals are relevant. A sensor defines which parameters it can physically produce. The compatibility engine intersects those sets and presents the user with valid mapping choices.

## Expansion

Additional Maintain.ai machine profiles should be added from the authoritative machine-profile definition rather than invented in the DeviceOS UI. DeviceOS should remain capable of consuming a larger registry without a schema redesign.
