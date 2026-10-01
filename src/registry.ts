export type BoardDefinition = {
  id: string;
  name: string;
  fqbn: string;
  digital_pins: number[];
  analog_pins: number[];
};

export type SensorDefinition = {
  id: string;
  name: string;
  protocol: string;
  parameters: string[];
  required_digital_pins: number;
  required_analog_pins: number;
};

export const initialBoards: BoardDefinition[] = [
  {
    id: 'arduino-uno-r3',
    name: 'Arduino Uno R3',
    fqbn: 'arduino:avr:uno',
    digital_pins: Array.from({ length: 14 }, (_, index) => index),
    analog_pins: [14, 15, 16, 17, 18, 19],
  },
  {
    id: 'arduino-nano',
    name: 'Arduino Nano',
    fqbn: 'arduino:avr:nano',
    digital_pins: Array.from({ length: 14 }, (_, index) => index),
    analog_pins: [14, 15, 16, 17, 18, 19],
  },
];

export const initialSensors: SensorDefinition[] = [
  { id: 'dht11', name: 'DHT11', protocol: 'single-wire', parameters: ['temperature', 'humidity'], required_digital_pins: 1, required_analog_pins: 0 },
  { id: 'sw420', name: 'SW-420 Vibration', protocol: 'digital', parameters: ['vibration'], required_digital_pins: 1, required_analog_pins: 0 },
  { id: 'generic-analog-input', name: 'Generic Analog Input', protocol: 'analog', parameters: ['analog'], required_digital_pins: 0, required_analog_pins: 1 },
  { id: 'generic-digital-input', name: 'Generic Digital Input', protocol: 'digital', parameters: ['digital'], required_digital_pins: 1, required_analog_pins: 0 },
];
