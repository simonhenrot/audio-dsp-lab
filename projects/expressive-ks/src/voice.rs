use rand::Rng;

use crate::dsp::{clamp, OnePole};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VoiceParams {
    pub brightness: f32,
    pub damping: f32,
    pub excitation_level: f32,
    pub pickup_position: f32,
    pub vibrato_depth: f32,
    pub vibrato_rate_hz: f32,
}

impl Default for VoiceParams {
    fn default() -> Self {
        Self {
            brightness: 0.7,
            damping: 0.15,
            excitation_level: 0.8,
            pickup_position: 0.5,
            vibrato_depth: 0.0,
            vibrato_rate_hz: 5.0,
        }
    }
}

pub struct KarplusStrongVoice {
    sample_rate: f32,
    active: bool,
    note: u8,
    velocity: f32,

    delay: Vec<f32>,
    write_idx: usize,
    delay_len: f32, // fractional target delay

    loop_filter: OnePole,
    amp_env: f32,
    decay_coef: f32,

    params: VoiceParams,
    age_samples: u64,
}

impl KarplusStrongVoice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            active: false,
            note: 0,
            velocity: 0.0,
            delay: vec![0.0; 256],
            write_idx: 0,
            delay_len: 64.0,
            loop_filter: OnePole::new(),
            amp_env: 0.0,
            decay_coef: 0.9999,
            params: VoiceParams::default(),
            age_samples: 0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn note(&self) -> u8 {
        self.note
    }

    pub fn age_samples(&self) -> u64 {
        self.age_samples
    }

    pub fn note_on(&mut self, note: u8, freq: f32, velocity: u8, params: VoiceParams) {
        self.note = note;
        self.velocity = velocity as f32 / 127.0;
        self.params = params.clone();
        self.active = true;
        self.age_samples = 0;
        self.amp_env = 1.0;

        let base_delay = self.sample_rate / freq;
        self.delay_len = base_delay.max(2.0);

        let buffer_len = self.delay_len.ceil() as usize + 2;
        self.delay.resize(buffer_len.max(4), 0.0);
        self.write_idx = 0;

        self.loop_filter
            .set_lowpass_coeff_from_brightness(self.params.brightness);

        let damping = clamp(self.params.damping, 0.0, 1.0);
        self.decay_coef = 0.9995 - 0.015 * damping;

        let excitation_gain = self.params.excitation_level * (0.3 + 0.7 * self.velocity);

        let mut rng = rand::thread_rng();
        for s in self.delay.iter_mut() {
            let noise: f32 = rng.gen_range(-1.0..1.0);
            *s = excitation_gain * noise;
        }
    }

    pub fn note_off(&mut self) {
        self.decay_coef *= 0.995;
    }

    pub fn set_brightness(&mut self, x: f32) {
        self.params.brightness = clamp(x, 0.0, 1.0);
        self.loop_filter
            .set_lowpass_coeff_from_brightness(self.params.brightness);
    }

    pub fn set_damping(&mut self, x: f32) {
        self.params.damping = clamp(x, 0.0, 1.0);
        self.decay_coef = 0.9995 - 0.02 * self.params.damping;
    }

    pub fn process(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }

        let len = self.delay.len();
        if len < 4 {
            self.active = false;
            return 0.0;
        }

        let read_pos = (self.write_idx as f32 - self.delay_len).rem_euclid(len as f32);
        let i0 = read_pos.floor() as usize;
        let frac = read_pos - i0 as f32;
        let i1 = (i0 + 1) % len;

        let delayed = self.delay[i0] * (1.0 - frac) + self.delay[i1] * frac;
        let filtered = self.loop_filter.process(delayed);
        let new_sample = filtered * self.decay_coef;

        self.delay[self.write_idx] = new_sample;
        self.write_idx = (self.write_idx + 1) % len;

        self.age_samples += 1;
        self.amp_env *= 0.999995;

        let out = delayed * self.amp_env;

        if out.abs() < 1.0e-5 && self.age_samples > (self.sample_rate as u64) {
            self.active = false;
        }

        out
    }
}
