use crate::dsp::{clamp, midi_note_to_freq, BiquadFilter};
use crate::midi::MidiMessage;
use crate::voice::{KarplusStrongVoice, VoiceParams};


pub struct Synth {
    voices: Vec<KarplusStrongVoice>,
    sample_rate: f32,
    global_brightness: f32,
    global_damping: f32,
    global_excitation: f32,
    pitch_bend_semitones: f32,
    morph: f32,           // <- nouveau : axe de morphing [0,1]
    body_filter: BiquadFilter,  // <- nouveau
}


impl Synth {
    pub fn new(sample_rate: f32, polyphony: usize) -> Self {
        let voices = (0..polyphony)
            .map(|_| KarplusStrongVoice::new(sample_rate))
            .collect();

        let mut body_filter = BiquadFilter::new();
            body_filter.set_resonant_peak(180.0, 1.2, 4.0, sample_rate);

        Self {
            voices,
            sample_rate,
            global_brightness: 0.6,
            global_damping: 0.05,
            global_excitation: 0.7,
            pitch_bend_semitones: 0.0,
            morph: 0.0,
            body_filter,
        }

    }

    fn make_params(&self) -> VoiceParams {
        VoiceParams {
            brightness: self.global_brightness,
            damping: self.global_damping,
            excitation_level: self.global_excitation,
            pickup_position: 0.5,
            vibrato_depth: 0.0,
            vibrato_rate_hz: 5.0,
        }
    }

    fn allocate_voice(&mut self) -> usize {
        if let Some((idx, _)) = self.voices.iter().enumerate().find(|(_, v)| !v.is_active()) {
            return idx;
        }

        self.voices
            .iter()
            .enumerate()
            .max_by_key(|(_, v)| v.age_samples())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    pub fn handle_midi(&mut self, msg: MidiMessage) {
        match msg {
            MidiMessage::NoteOn { note, vel, .. } => {
                let idx = self.allocate_voice();
                let freq = midi_note_to_freq(note) * 2.0_f32.powf(self.pitch_bend_semitones / 12.0);

                let mut params = self.make_params();

                // vélocité -> excitation et brillance légèrement augmentées
                let v = vel as f32 / 127.0;
                params.excitation_level = clamp(0.3 + 0.9 * v, 0.0, 1.0);
                params.brightness = clamp(self.global_brightness * (0.8 + 0.3 * v), 0.0, 1.0);

                self.voices[idx].note_on(note, freq, vel, params);
                self.voices[idx].apply_morph(self.morph);
            }

            MidiMessage::NoteOff { note, .. } => {
                for voice in self
                    .voices
                    .iter_mut()
                    .filter(|v| v.is_active() && v.note() == note)
                {
                    voice.note_off();
                }
            }

            MidiMessage::ControlChange { cc, val, .. } => {
                let x = val as f32 / 127.0;
                match cc {

                    // CC17 : pression verticale haut -> morphing UNIQUEMENT
                    17 => {
                        self.morph = x;
                        //println!("morph -> {:.3}", self.morph); // debug temporaire
                        for v in self.voices.iter_mut() {
                            if v.is_active() {
                                v.apply_morph(self.morph);
                            }
                        }
                    }

                    // CC16, CC18, CC19 : ignorés pendant le debug
                    16 | 18 | 19 => {}

                    // Mappings clavier conservés
                    1 => {
                        self.global_brightness = x;
                        for v in self.voices.iter_mut() {
                            if v.is_active() { v.set_brightness(x); }
                        }
                    }
                    74 => {
                        self.global_damping = 1.0 - x;
                        for v in self.voices.iter_mut() {
                            if v.is_active() { v.set_damping(1.0 - x); }
                        }
                    }
                    71 => { self.global_excitation = x; }
                    _ => {}
                }
            }




            MidiMessage::PitchBend { value, .. } => {
                // plage +/- 2 demi-tons
                self.pitch_bend_semitones = 2.0 * (value as f32 / 8192.0);
            }
        }
    }

    pub fn process(&mut self) -> f32 {
        let mut y = 0.0;
        for v in &mut self.voices {
            y += v.process();
        }
        let y = self.body_filter.process(0.2 * y);
        y.tanh()
    }


    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
}
