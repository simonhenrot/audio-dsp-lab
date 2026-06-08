mod dsp;
mod midi;
mod synth;
mod voice;

use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{unbounded, Receiver};
use midir::{Ignore, MidiInput};

use crate::midi::parse_midi;
use crate::synth::Synth;

fn main() -> Result<()> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow!("No default audio output device found"))?;

    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    println!("Audio output: {}", device.name()?);
    println!("Sample rate: {}", sample_rate);
    println!("Channels: {}", channels);

    let (midi_tx, midi_rx) = unbounded();

    let mut midi_in = MidiInput::new("ks-synth-midi-in")?;
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    if in_ports.is_empty() {
        println!("No MIDI input ports found. Start anyway without MIDI.");
    }

    let _midi_conn = if !in_ports.is_empty() {
        println!("Available MIDI input ports:");
        for (i, port) in in_ports.iter().enumerate() {
            println!("  [{}] {}", i, midi_in.port_name(port)?);
        }

        println!("Select MIDI input port number: ");
        let selected_index = read_usize_from_stdin()?;

        if selected_index >= in_ports.len() {
            return Err(anyhow!("MIDI port index out of range"));
        }

        let port = &in_ports[selected_index];
        println!("Using MIDI input: {}", midi_in.port_name(port)?);

        Some(midi_in.connect(
            port,
            "ks-synth-read-input",
            move |_stamp, message, _| {
                if let Some(msg) = parse_midi(message) {
                    // println!("MIDI: {:?}", msg);
                    let _ = midi_tx.send(msg);
                }
            },
            (),
        )?)
    } else {
        None
    };

    match config.sample_format() {
        cpal::SampleFormat::F32 => {
            run_stream::<f32>(&device, &config.into(), sample_rate, channels, midi_rx)?
        }
        cpal::SampleFormat::I16 => {
            run_stream::<i16>(&device, &config.into(), sample_rate, channels, midi_rx)?
        }
        cpal::SampleFormat::U16 => {
            run_stream::<u16>(&device, &config.into(), sample_rate, channels, midi_rx)?
        }
        other => return Err(anyhow!("Unsupported sample format: {:?}", other)),
    }

    Ok(())
}

fn run_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    sample_rate: f32,
    channels: usize,
    midi_rx: Receiver<midi::MidiMessage>,
) -> Result<()>
where
    T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f32>,
{
    let mut synth = Synth::new(sample_rate, 16);

    let err_fn = |err| eprintln!("Audio stream error: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            while let Ok(msg) = midi_rx.try_recv() {
                synth.handle_midi(msg);
            }

            for frame in data.chunks_mut(channels) {
                let sample = synth.process();

                let value: T = T::from_sample(sample);
                for out in frame.iter_mut() {
                    *out = value;
                }
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    println!("Running. Press Enter to quit.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}

fn read_usize_from_stdin() -> Result<usize> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let value = input
        .trim()
        .parse::<usize>()
        .map_err(|_| anyhow!("Invalid number"))?;
    Ok(value)
}
