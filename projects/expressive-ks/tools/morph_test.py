"""
Diagnostic script: Karplus-Strong with morph axis, no MIDI, exports WAV.
Tests whether morph parameter actually extends note duration.

Usage:
    python tools/morph_test.py

Output:
    morph_test_morph0.wav   -- morph=0.0 (short pluck)
    morph_test_morph05.wav  -- morph=0.5 (mid)
    morph_test_morph1.wav   -- morph=1.0 (long sustained)
"""

import numpy as np
import wave
import struct

SAMPLE_RATE = 48000
DURATION_S  = 4.0
N_SAMPLES   = int(SAMPLE_RATE * DURATION_S)


def midi_to_freq(note: int) -> float:
    return 440.0 * 2.0 ** ((note - 69) / 12.0)


def one_pole_coeff(brightness: float) -> float:
    return 0.05 + 0.94 * brightness


def apply_morph(morph: float) -> dict:
    """Mirrors the Rust apply_morph logic exactly."""
    morph = float(np.clip(morph, 0.0, 1.0))

    damping_a    = 0.25
    brightness_a = 0.5
    decay_a      = 0.9985

    damping_b    = 0.01
    brightness_b = 0.85
    decay_b      = 0.99998

    damping    = damping_a    + morph * (damping_b    - damping_a)
    brightness = brightness_a + morph * (brightness_b - brightness_a)
    decay      = decay_a      + morph * (decay_b      - decay_a)
    decay_coef = decay - 0.015 * damping

    return {
        "damping":    damping,
        "brightness": brightness,
        "decay_coef": decay_coef,
        "loop_a":     one_pole_coeff(brightness),
    }


def synthesize(note: int, morph: float, velocity: float = 0.8) -> np.ndarray:
    params     = apply_morph(morph)
    decay_coef = params["decay_coef"]
    loop_a     = params["loop_a"]
    freq       = midi_to_freq(note)
    delay_len  = SAMPLE_RATE / freq

    print(f"  morph={morph:.2f} | decay_coef={decay_coef:.6f} | "
          f"brightness={params['brightness']:.3f} | "
          f"delay_len={delay_len:.1f} samples")

    # Théorique : durée à -60dB
    if decay_coef < 1.0:
        t60 = -60.0 / (20.0 * np.log10(decay_coef) * SAMPLE_RATE / delay_len)
        print(f"  Theoretical T60 ≈ {t60:.2f} s")
    else:
        print(f"  decay_coef >= 1.0 : voix ne s'éteint pas")

    buf_len   = int(np.ceil(delay_len)) + 2
    delay_buf = np.zeros(buf_len)

    # Excitation colorée
    rng        = np.random.default_rng(seed=42)
    brightness_coeff = 0.1 + 0.85 * params["brightness"]
    filtered_noise   = 0.0
    excitation_gain  = velocity * 0.8

    for i in range(buf_len):
        white          = rng.uniform(-1.0, 1.0)
        filtered_noise = brightness_coeff * white + (1.0 - brightness_coeff) * filtered_noise
        pos            = i / buf_len
        shape          = np.sin(np.pi * pos)
        decay_env      = np.exp(-4.0 * pos)
        delay_buf[i]   = excitation_gain * filtered_noise * shape * decay_env

    # Boost énergie si morph > 0.3 (miroir du Rust)
    if morph > 0.3:
        boost = 1.0 + 0.08 * (morph - 0.3)
        delay_buf *= boost

    output    = np.zeros(N_SAMPLES)
    write_idx = 0
    z1        = 0.0
    amp_env   = 1.0

    for n in range(N_SAMPLES):
        # Lecture fractionnaire
        read_pos = (write_idx - delay_len) % buf_len
        i0       = int(read_pos)
        frac     = read_pos - i0
        i1       = (i0 + 1) % buf_len
        delayed  = delay_buf[i0] * (1.0 - frac) + delay_buf[i1] * frac

        # One-pole loop filter
        z1       = loop_a * delayed + (1.0 - loop_a) * z1
        filtered = z1

        # Feedback avec morph (miroir du Rust)
        effective_decay = min(decay_coef + 0.0008 * morph, 0.99999)
        new_sample      = filtered * effective_decay

        delay_buf[write_idx] = new_sample
        write_idx = (write_idx + 1) % buf_len

        amp_env   *= 0.999995
        output[n]  = delayed * amp_env

        # Seuil d'extinction (miroir du Rust)
        # if abs(output[n]) < 1e-5 and n > SAMPLE_RATE:
        if abs(output[n]) < 1e-15 and n > SAMPLE_RATE:
            print(f"  Voice died at sample {n} = {n/SAMPLE_RATE:.3f} s")
            break

    return output


def write_wav(filename: str, samples: np.ndarray) -> None:
    # Normalise et exporte en WAV 16-bit
    peak = np.max(np.abs(samples))
    if peak > 0:
        samples = samples / peak * 0.9
    int_samples = (samples * 32767).astype(np.int16)

    with wave.open(filename, 'w') as f:
        f.setnchannels(1)
        f.setsampwidth(2)
        f.setframerate(SAMPLE_RATE)
        f.writeframes(int_samples.tobytes())

    print(f"  -> {filename}")


if __name__ == "__main__":
    note = 60  # C4

    for morph in [0.0, 0.5, 1.0]:
        print(f"\n--- morph = {morph} ---")
        audio    = synthesize(note, morph)
        filename = f"morph_test_morph{str(morph).replace('.', '')}_seuil_1e-15.wav"
        write_wav(filename, audio)

    print("\nDone. Compare the three WAV files.")
    print("If morph=1.0 is not longer than morph=0.0, the bug is in the model.")
    print("If morph=1.0 is longer, the bug is in the MIDI routing or the Rust code.")
