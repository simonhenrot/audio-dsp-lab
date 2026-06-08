
---

## `projects/expressive-ks/docs/design-notes.md`

```md
# Design notes

## Purpose

`expressive-ks` is a lightweight research prototype for exploring expressive control in physically inspired sound synthesis.

The first implementation focuses on a Karplus-Strong plucked-string model because it is simple, computationally efficient, and immediately useful for studying the relationship between control gestures and timbral variation.

## Synthesis model

The current synthesis engine is based on a standard Karplus-Strong architecture:

- noise-burst excitation
- delay-line feedback loop
- simple loop low-pass filtering
- damping through feedback attenuation
- one voice per active note
- polyphonic voice allocation at synth level

This model is intentionally minimal. It is not meant to emulate a specific acoustic instrument with high realism, but to provide a controllable and musically meaningful baseline.

## Why Karplus-Strong first

Karplus-Strong is a good first step for this project because it provides:

- a clear link between physical intuition and DSP structure
- low CPU cost
- a compact implementation
- fast iteration for testing expressive mappings
- an easy path toward richer waveguide-based models

For a first GitHub-facing prototype, it strikes a good balance between technical seriousness and implementation speed.

## Audio architecture

The current runtime is a standalone Rust application with:

- real-time audio output
- a polyphonic synth engine
- MIDI input
- per-sample voice processing
- lightweight summation and output soft clipping

The implementation favors simplicity, readability and direct experimentation over abstraction-heavy design.

## Voice model

Each voice currently contains:

- a delay buffer
- a write pointer
- a target delay length derived from note frequency
- a one-pole loop filter
- a decay coefficient
- a simple amplitude decay state
- a small parameter set for timbre control

Voice stealing currently uses a simple age-based strategy when all voices are active.

## Expressive control strategy

The project is specifically interested in mapping external gesture streams to perceptually meaningful synthesis parameters.

At this stage, control remains global rather than per-note. The current parameters exposed to control are:

- **brightness**
- **damping**
- **excitation level**
- **global pitch bend**

This is enough to validate a first gesture-to-timbre interaction loop.

## Real-time constraints

The implementation is intended to remain compatible with real-time audio constraints:

- lightweight per-voice state
- no heavy computation inside the synthesis loop
- simple filters and feedback structure
- no dependence on offline preprocessing
- minimal architecture overhead

The code is still a prototype and may require further review for strict real-time safety, but the overall direction is aligned with low-latency standalone audio execution.

## Current limitations

The current prototype has several intentional limitations:

- only one MIDI port can be used at a time
- controller information is global, not per-note
- pitch bend affects newly triggered notes but not active voices continuously
- excitation is based on simple random initialization
- no body resonator or radiation model is implemented
- no plugin format is provided yet

These limitations are acceptable for a V1 whose main goal is to validate the architectural baseline.

## Next technical steps

The most relevant short-term improvements are:

- support two simultaneous MIDI ports
- separate note input from expressive control input
- add explicit mapping for Touché SE
- improve excitation shaping
- add a simple body filter or resonator stage
- support retuning of active voices during pitch bend
- prepare for per-note control and MPE-oriented evolution

## Broader direction

In a more advanced version, this project could evolve toward:

- digital waveguide variants
- modal or hybrid resonator models
- nonlinear excitation models
- perceptually informed controller mappings
- evaluation of expressive parameter spaces
- embedded-friendly implementations

The long-term research value of the project lies less in the raw Karplus-Strong algorithm than in the study of how expressive gesture interacts with physically meaningful sound-generation parameters.
