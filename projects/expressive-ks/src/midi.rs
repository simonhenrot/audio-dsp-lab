#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MidiMessage {
    NoteOn { ch: u8, note: u8, vel: u8 },
    NoteOff { ch: u8, note: u8 },
    ControlChange { ch: u8, cc: u8, val: u8 },
    PitchBend { ch: u8, value: i16 },
}

pub fn parse_midi(message: &[u8]) -> Option<MidiMessage> {
    if message.is_empty() {
        return None;
    }

    let status = message[0];
    let msg_type = status & 0xF0;
    let ch = status & 0x0F;

    match msg_type {
        0x80 if message.len() >= 3 => Some(MidiMessage::NoteOff {
            ch,
            note: message[1],
        }),
        0x90 if message.len() >= 3 => {
            let note = message[1];
            let vel = message[2];
            if vel == 0 {
                Some(MidiMessage::NoteOff { ch, note })
            } else {
                Some(MidiMessage::NoteOn { ch, note, vel })
            }
        }
        0xB0 if message.len() >= 3 => Some(MidiMessage::ControlChange {
            ch,
            cc: message[1],
            val: message[2],
        }),
        0xE0 if message.len() >= 3 => {
            let lsb = message[1] as i16;
            let msb = message[2] as i16;
            let raw = (msb << 7) | lsb;
            Some(MidiMessage::PitchBend {
                ch,
                value: raw - 8192,
            })
        }
        _ => None,
    }
}
