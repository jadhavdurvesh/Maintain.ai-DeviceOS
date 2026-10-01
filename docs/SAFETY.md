# Safety Model

DeviceOS interacts with physical electronics and may be used around industrial machinery.

## Upload safeguards

Before upload show:
- target machine
- target board
- sensors
- pin assignments
- wiring
- relevant warnings

Require explicit confirmation that wiring has been checked.

Block upload for invalid configuration, compile failure, board mismatch and unresolved conflicts.

Warn for reserved pins, voltage uncertainty, unsupported pairing, missing calibration, destructive replacement and credential regeneration.

## Auto-shutdown boundary

A software threshold is not a certified safety function.

Maintain.ai 3 contains application-level machine safety/auto-shutdown configuration. DeviceOS may acquire signals, configure device-side logic and report state, but actual emergency-stop or safety-rated functions require appropriate physical safety hardware and validated system architecture.

## Destructive operations

Firmware replacement, rollback, reset, key regeneration and deletion require explicit user action and local audit logging.
