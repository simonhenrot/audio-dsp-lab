# Roadmap

## V1 - First playable prototype

Goal: establish a clean and functional baseline.

### Scope

- standalone Rust application
- real-time audio output
- polyphonic Karplus-Strong plucked-string synthesis
- MIDI note input
- global MIDI CC control
- interactive MIDI input selection
- initial project documentation

### Expected outcome

A first playable prototype demonstrating that the core architecture works end-to-end:
MIDI input -> control mapping -> physical-modeling voice engine -> real-time audio output.

## V1.1 - Expressive controller integration

Goal: turn the prototype into a more relevant expressive instrument demonstrator.

### Scope

- dual MIDI input support
- one note controller plus one expressive controller
- explicit Touché SE mapping
- improved parameter mapping for gesture-to-timbre control
- first demo audio recordings
- screenshots, diagrams or spectrograms

### Expected outcome

A musically clearer demonstration of controller-aware synthesis, showing how continuous gesture can shape a physically inspired string model in real time.

## V2 - Improved synthesis quality

Goal: make the sound engine more convincing while keeping the code compact and readable.

### Scope

- improved excitation shaping
- support for pluck position
- better damping behavior
- lightweight body resonator or output filter
- continuous retuning of active voices
- cleaner internal parameter handling

### Expected outcome

A more expressive and more instrument-like plucked-string synth, still lightweight enough for rapid experimentation.

## V3 - Per-note expression and advanced control

Goal: move from global control to note-specific expressivity.

### Scope

- per-note control logic
- channel-aware MIDI handling
- partial or full MPE support
- better voice/controller association
- evaluation of multidimensional control strategies

### Expected outcome

A much more relevant platform for studying expressive digital instrument interaction beyond conventional MIDI keyboard behavior.

## Longer-term directions

Possible future directions include:

- digital waveguide refinements
- modal synthesis variants
- hybrid physical models
- embedded-oriented optimization
- plugin version
- perceptual evaluation of controller mappings
- comparison of mappings across different expressive controllers

## Development philosophy

The project will evolve incrementally:

- first make it playable
- then make it expressive
- then make it richer
- only then make it more complex

This ordering is intentional. The goal is not to build the most elaborate model as fast as possible, but to build a clear and credible prototype platform for expressive synthesis research.
