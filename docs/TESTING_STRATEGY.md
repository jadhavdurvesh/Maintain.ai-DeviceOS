# Testing Strategy

## Unit tests

- board definitions
- sensor definitions
- configuration schema
- pin assignment
- conflict detection
- wiring generation
- firmware generation
- telemetry serialization

## Integration

```text
configuration
→ generation
→ Arduino CLI compile
→ artifact
→ upload adapter
→ handshake parser
```

Mock serial hardware in automated tests.

## Hardware matrix

MVP:
- Arduino Uno
- Arduino Nano
- DHT11
- SW-420
- analog input
- digital input

## Golden builds

Known configuration fixtures should generate deterministic source apart from intentionally variable metadata.

## Failure tests

- device disconnected
- port disappears during upload
- wrong board
- compiler error
- missing library
- pin conflict
- unsupported pin
- no handshake
- malformed serial message

## Maintain.ai integration

Verify telemetry maps to the intended machine and signal without unexpectedly creating or mutating maintenance records.
