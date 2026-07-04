# Touché SE mapping

## Confirmed axis assignment (tested 2026-06-22)

Axis assignment determined empirically using a MIDI monitoring device.

| CC  | Physical axis             | Range   |
|-----|---------------------------|---------|
| CC16 | Vertical pressure (bottom) | 0-127  |
| CC17 | Vertical pressure (top)    | 0-127  |
| CC18 | Left lateral axis          | 0-127  |
| CC19 | Right lateral axis         | 0-127  |

Note: CC18 (left lateral axis) shows reduced sensitivity compared to other
axes. To be investigated in a future calibration session.

## Current synthesis mapping

| CC  | Physical axis              | Synthesis parameter  | Effect                        |
|-----|----------------------------|----------------------|-------------------------------|
| CC16 | Vertical pressure (bottom) | Brightness           | Bright <-> dark timbre        |
| CC17 | Vertical pressure (top)    | Damping              | Long <-> short sustain        |
| CC18 | Left lateral axis          | Excitation level     | Strong <-> soft attack        |
| CC19 | Right lateral axis         | Brightness modulation| Subtle tonal colour           |

## Known issues

- CC18 sensitivity appears lower than the other axes — mapping may need
  rescaling once the hardware issue is better understood
- Mapping to be refined based on musical use and listening tests

## Rationale

Pressure axes are mapped to parameters that shape energy and attack,
lateral axes to spectral character. This follows an intuition that
vertical pressure maps naturally to tonal intensity, and lateral motion
to timbral colour.
