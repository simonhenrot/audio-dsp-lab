# expressive-ks

A real-time Rust prototype for expressive plucked-string synthesis based on a Karplus-Strong physical model.

## Overview

`expressive-ks` is a standalone audio synthesizer exploring the connection between **physically inspired sound synthesis** and **continuous expressive control**.

The current version implements a lightweight **polyphonic plucked-string model** in Rust, with real-time audio output and MIDI control. The project is intended as a research and prototyping platform for studying how gesture can shape timbre in computationally efficient physical-modeling instruments.

## Motivation

This project is not only about implementing a classic Karplus-Strong string model. Its broader goal is to investigate a more interesting question:

**How can continuous control signals from expressive controllers be mapped to physically meaningful synthesis parameters in a musically relevant and real-time-compatible way?**

This direction is particularly relevant for digital musical instruments where gesture quality matters as much as synthesis quality.

## Current features

- Real-time standalone audio output
- Polyphonic plucked-string synthesis
- Karplus-Strong delay-line model
- MIDI note input
- Global expressive control through MIDI CC
- Interactive MIDI input port selection at startup
- Simple and extensible DSP architecture

## Current control mapping

The current prototype supports the following controls:

| Control | Function |
|---|---|
| Note On / Note Off | Trigger and release plucked-string voices |
| Pitch Bend | Global tuning offset |
| CC1 | Spectral brightness |
| CC74 | Damping / timbre control |
| CC71 | Excitation level |

## Project structure

| Path | Role |
|---|---|
| `src/` | Rust source code for audio, MIDI and synthesis |
| `docs/design-notes.md` | Technical design notes |
| `docs/roadmap.md` | Development roadmap |
| `docs/experiments.md` | Notes for future experiments and listening tests |
| `assets/audio/` | Demo audio files |
| `assets/img/` | Diagrams, screenshots or spectrograms |
| `assets/midi/` | Controller mapping notes |

## Build and run

From the `projects/expressive-ks` directory:

```bash
cargo run --release
