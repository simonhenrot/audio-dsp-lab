mod dsp;
mod midi;
mod synth;
mod voice;

use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{unbounded, Receiver};
use midir::{Ignore, MidiInput, MidiInputConnection};

use crate::midi::parse_midi;
use crate::synth::Synth;

fn main() -> Result<()> {
    let host = cpal::default_host();

    //let device = host
    //    .default_output_device()
    //    .ok_or_else(|| anyhow!("No default audio output device found"))?;

    println!("Available audio output devices:");
    for d in host.output_devices()? {
        if let Ok(name) = d.name() {
            println!("  {}", name);
        }
    }


    let device = {
        let mut found = None;
        for d in host.output_devices()? {
            if let Ok(name) = d.name() {
                println!("  Audio device: {}", name);
                if name.contains("BlackHole") {
                    found = Some(d);
                    break;
                }
            }
        }
        found.ok_or_else(|| anyhow!("BlackHole audio device not found"))?
    };


    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;
    let channels = channels.min(2); // forcer stéréo

    println!("Audio output: {}", device.name()?);
    println!("Sample rate: {}", sample_rate);
    println!("Channels: {}", channels);

    let (midi_tx, midi_rx) = unbounded();

    let mut midi_in = MidiInput::new("expressive-ks-midi-in")?;
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    if in_ports.is_empty() {
        return Err(anyhow!("No MIDI input ports found"));
    }

    println!("Available MIDI input ports:");
    for (i, port) in in_ports.iter().enumerate() {
        println!("  [{}] {}", i, midi_in.port_name(port)?);
    }

    println!("Select note MIDI input port number: ");
    let note_port_index = read_usize_from_stdin()?;
    if note_port_index >= in_ports.len() {
        return Err(anyhow!("Note MIDI port index out of range"));
    }

    println!("Select expression MIDI input port number: ");
    let expr_port_index = read_usize_from_stdin()?;
    if expr_port_index >= in_ports.len() {
        return Err(anyhow!("Expression MIDI port index out of range"));
    }

    let note_port_name = midi_in.port_name(&in_ports[note_port_index])?;
    let expr_port_name = midi_in.port_name(&in_ports[expr_port_index])?;

    println!("Using note input: {}", note_port_name);
    println!("Using expression input: {}", expr_port_name);

    let note_conn = connect_midi_port(
        "expressive-ks-note-input",
        &in_ports[note_port_index],
        midi_tx.clone(),
        false,
    )?;

    let expr_conn = connect_midi_port(
        "expressive-ks-expression-input",
        &in_ports[expr_port_index],
        midi_tx.clone(),
        false,
        //true,
    )?;

    let _midi_connections = vec![note_conn, expr_conn];

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

fn connect_midi_port(
    connection_name: &str,
    port: &midir::MidiInputPort,
    midi_tx: crossbeam_channel::Sender<midi::MidiMessage>,
    debug_log: bool,
) -> Result<MidiInputConnection<()>> {
    let mut midi_in = MidiInput::new(connection_name)?;
    midi_in.ignore(Ignore::None);

    let callback_name = connection_name.to_owned();

    let conn = midi_in.connect(
        port,
        connection_name,
        move |_stamp, message, _| {
            if let Some(msg) = parse_midi(message) {
                if debug_log {
                    println!("MIDI [{}]: {:?}", callback_name, msg);
                }
                let _ = midi_tx.send(msg);
            }
        },
        (),
    )?;

    Ok(conn)
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
