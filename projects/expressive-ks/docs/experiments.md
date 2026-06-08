# Experiments

## Purpose

This file collects ideas for technical and musical experiments around the `expressive-ks` prototype.

The goal is to document promising directions, keep track of listening-test ideas, and identify which changes improve the instrument in meaningful ways.

## Experiment themes

### 1. Excitation design

Study how the initial excitation affects perceived attack, brightness and realism.

Ideas:
- white-noise burst
- filtered noise burst
- velocity-dependent excitation shaping
- pluck-position-dependent excitation profile

Questions:
- Which excitation model gives the clearest perceptual improvement?
- Which variants remain computationally lightweight?
- How strongly does the excitation influence controller responsiveness?

### 2. Gesture-to-timbre mapping

Study how continuous controller dimensions should be mapped to synthesis parameters.

Candidate targets:
- excitation level
- loop-filter brightness
- damping
- pluck position
- body resonance
- vibrato depth

Questions:
- Which mappings feel physically intuitive?
- Which mappings sound expressive rather than arbitrary?
- Which mappings remain stable under real-time performance conditions?

### 3. Controller comparison

Compare different control sources.

Candidate controllers:
- standard MIDI keyboard
- Touché SE
- MPE controller
- automation from a DAW

Questions:
- What is lost when using a keyboard only?
- What becomes musically useful when continuous gesture is added?
- Which controller dimensions are the most perceptually valuable?

### 4. Sound quality improvements

Evaluate low-cost improvements to the current synthesis engine.

Ideas:
- body filter
- dispersion approximation
- better fractional delay handling
- output coloration
- stereo widening through resonator variation

Questions:
- Which additions provide the best perceptual return for the added complexity?
- Which ones remain suitable for lightweight real-time execution?

### 5. Performance and implementation

Track implementation choices affecting runtime stability and CPU cost.

Ideas:
- avoid unnecessary allocations
- benchmark polyphony limits
- compare debug vs release behavior
- inspect audio stability under dense note triggering

Questions:
- What is the practical polyphony ceiling?
- Which parts of the code dominate runtime cost?
- Which features threaten real-time robustness?

## Listening tests

Useful listening-test scenarios:

- repeated notes with different excitation settings
- same phrase with different damping mappings
- same phrase with and without external expressive control
- slow controller sweeps over sustained notes
- comparison between discrete and continuous timbre control

## Documentation to add later

To make experiments more useful over time, add:

- short audio examples
- spectrogram screenshots
- notes on perceived behavior
- CPU observations
- controller configuration details

## Guiding principle

An experiment is only worth keeping if it improves at least one of these dimensions:

- musical expressivity
- perceptual clarity
- physical plausibility
- implementation simplicity
- real-time suitability
