# Spatial mapping design

## Perceptual model

The mapping is grounded in psychoacoustic distance cues:

- **Reverb level**: more reverb = more distant (room effect)
- **Low-pass filtering**: high frequencies attenuate with distance in air
- **Combined**: reverb + LPF together produce a convincing depth impression

Horizontal position is controlled by stereo panning, the most direct and
perceptually stable spatial dimension.

## Current Touché axis assignment

| CC  | Touché axis               | Mapped parameter         | Direction         |
|-----|---------------------------|--------------------------|-------------------|
| CC16 | Vertical pressure (bottom) | Reverb send level       | push = more reverb |
| CC17 | Vertical pressure (top)    | Low-pass filter cutoff  | push = darker      |
| CC18 | Left lateral               | Panning (left)          | lean = pan left    |
| CC19 | Right lateral              | Panning (right)         | lean = pan right   |

## Implementation notes

Two MIDI channels are used for the two pressure axes and two lateral axes,
matching the Touché SE's dual-channel output.

## Perceptual rationale

- Vertical pressure maps intuitively to depth: pressing down = sinking back
  into the space
- Lateral gesture maps intuitively to position: leaning left = sound moves left
- The combination produces a 2D spatial gesture vocabulary

## Planned extensions

### Quadraphonic

Add rear channels, map front/back pressure to front/rear balance.

### Elevation (dome / ambisonics)

Use a second expressive controller or aftertouch for height.

### HRTF binaural

Replace panning with HRTF convolution for headphone listening.

### Gesture memory

Record spatial trajectories and play them back as automation.

## Research direction

The deeper question this project asks is:

*Can continuous gesture produce spatial trajectories that feel composed
rather than automated — and what mapping strategies make that possible?*

This connects to broader questions in embodied music cognition and
the design of expressive digital instruments.
