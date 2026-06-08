# Audio DSP Lab

This repository contains a collection of small, self-contained projects focused on audio signal processing and real-time DSP constraints.

## Projects

### expressive-ks

A real-time Rust prototype for expressive plucked-string synthesis based on a Karplus-Strong physical model.

Path: `projects/expressive-ks/`

Main focus:
- physical-modeling sound synthesis
- real-time audio DSP
- expressive controller mapping
- future Touché SE / MPE integration


### 1. Speech Denoising

* Classical DSP (STFT, Wiener filtering)
* Lightweight neural network approach
* Focus on low-latency processing

### 2. Pitch Tracking

* Fundamental frequency estimation (YIN / autocorrelation)
* Real-time capable implementation

### 3. Source Separation

* Simple spectrogram masking / NMF-based separation
* Educational implementation with clear assumptions



## Goals

* Bridge theory and practical audio DSP
* Explore real-time and embedded constraints
* Provide reproducible and lightweight implementations

## Tech Stack

Python, NumPy, SciPy, Librosa, PyTorch

## Author

Simon Henrot
