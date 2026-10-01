# Recommended Repository Structure

```text
Maintain.ai-DeviceOS/
├── apps/
│   └── desktop/
├── packages/
│   ├── ui/
│   ├── schemas/
│   ├── board-registry/
│   ├── sensor-registry/
│   ├── firmware-engine/
│   └── telemetry/
├── native/
│   └── rust/
│       ├── serial/
│       ├── usb/
│       ├── compiler/
│       ├── uploader/
│       ├── filesystem/
│       └── commands/
├── firmware/
│   ├── core/
│   ├── templates/
│   ├── sensors/
│   ├── generated/
│   └── tests/
├── database/
│   ├── migrations/
│   └── seeds/
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── fixtures/
│   └── hardware/
├── docs/
├── scripts/
└── assets/
```

The first implementation may start flatter; these boundaries should remain visible.
