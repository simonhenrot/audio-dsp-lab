pub fn midi_note_to_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

pub fn clamp(x: f32, lo: f32, hi: f32) -> f32 {
    x.max(lo).min(hi)
}

#[derive(Clone, Copy, Debug)]
pub struct OnePole {
    a: f32,
    z1: f32,
}

impl OnePole {
    pub fn new() -> Self {
        Self { a: 0.5, z1: 0.0 }
    }

    pub fn set_lowpass_coeff_from_brightness(&mut self, brightness: f32) {
        // brightness in [0,1]
        // low brightness => stronger smoothing
        // high brightness => less smoothing
        self.a = 0.05 + 0.94 * brightness;
    }

    pub fn process(&mut self, x: f32) -> f32 {
        self.z1 = self.a * x + (1.0 - self.a) * self.z1;
        self.z1
    }
}

/// Simple biquad filter — low-shelf / resonant peak for body simulation
#[derive(Clone, Copy, Debug)]
pub struct BiquadFilter {
    b0: f32, b1: f32, b2: f32,
    a1: f32, a2: f32,
    z1: f32, z2: f32,
}

impl BiquadFilter {
    pub fn new() -> Self {
        Self { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0, z1: 0.0, z2: 0.0 }
    }

    /// Résonant peak : booste une fréquence de corps simulée
    pub fn set_resonant_peak(
        &mut self,
        freq_hz: f32,
        q: f32,
        gain_db: f32,
        sample_rate: f32,
    ) {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * freq_hz / sample_rate;
        let alpha = w0.sin() / (2.0 * q);

        self.b0 = 1.0 + alpha * a;
        self.b1 = -2.0 * w0.cos();
        self.b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        self.a1 = -2.0 * w0.cos() / a0;
        self.a2 = (1.0 - alpha / a) / a0;
        self.b0 /= a0;
        self.b1 /= a0;
        self.b2 /= a0;
    }

    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y
    }
}
