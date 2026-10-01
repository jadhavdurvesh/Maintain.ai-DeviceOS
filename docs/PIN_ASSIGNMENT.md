# Pin Assignment Engine

## Goal

Automatically select safe compatible pins while allowing manual override through the same validator.

## Inputs

- board definition
- selected sensors
- reserved pins
- communication requirements
- existing assignments

## Rules

1. Match required capability before choosing a pin.
2. Reject incompatible reuse/conflicts.
3. Respect reserved UART/I2C/SPI pins unless explicitly supported.
4. Validate known voltage/protocol compatibility.
5. Use deterministic assignment.
6. Manual overrides go through identical validation.

## Example

Arduino Uno + DHT11 + SW-420:

DHT11 DATA → D7
SW-420 DO → D8

## Validation output

Return:
- valid
- warnings[]
- errors[]
- reserved[]
- rationale[]
