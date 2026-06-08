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
