# Firmware Engine

## Principle

DeviceOS does not generate arbitrary C/C++.

It composes tested firmware components and templates from a validated configuration.

## Pipeline

```text
Configuration
↓
Schema validation
↓
Template/module selection
↓
Source generation
↓
Arduino CLI compile
↓
HEX/BIN
↓
Upload
↓
Handshake
↓
Verification
```

## Template layout

```text
firmware/
├── core/
├── templates/
│   ├── arduino/
│   └── esp32/
└── sensors/
    ├── dht11/
    ├── sw420/
    └── generic/
```

## Reproducibility

Record:
- configuration hash
- template versions
- registry versions
- toolchain version
- source checksum
- output checksum

## Upload policy

Never upload when configuration is invalid, compilation fails, board mismatch is detected, mandatory dependencies are missing, or an unresolved pin conflict exists.

## Rollback

Retain previous validated builds and require explicit confirmation before replacing the current firmware.
