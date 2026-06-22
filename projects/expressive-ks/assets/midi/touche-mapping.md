# Touché SE mapping

## Observed CC output

The Touché SE sends continuous Control Change messages on four CC numbers:

- CC16 : likely vertical pressure (top)
- CC17 : likely right lateral axis
- CC18 : likely left lateral axis
- CC19 : likely vertical pressure (bottom)

Note: exact axis assignment is based on empirical observation and may vary
depending on the Touché preset in use.

## Current synthesis mapping

| CC | Axis (estimated) | Synthesis parameter | Effect |
|---|---|---|---|
| CC16 | Vertical pressure top | Brightness | Bright <-> dark timbre |
| CC17 | Right lateral | Damping | Long <-> short sustain |
| CC18 | Left lateral | Excitation level | Strong <-> soft attack |
| CC19 | Vertical pressure bottom | Brightness modulation | Subtle tonal colour |

## Rationale

The mapping connects gesture dimensions to perceptually salient parameters:
- pressure -> excitation and tonal energy
- lateral motion -> spectral character and sustain
- combined axes -> continuous morphing of the plucked string timbre

## Notes

- The Touché sends CC values continuously, even at rest, so some baseline
  drift is expected
- Mapping may need adjustment depending on the active Touché preset
- Per-note control is planned for a future version
