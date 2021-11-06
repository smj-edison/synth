#![allow(clippy::needless_range_loop)]

use std::error::Error;
use std::{io::Write, thread, time::Duration};

use simple_error::SimpleError;

use engine::constants::{BUFFER_SIZE, SAMPLE_RATE};

use engine::backend::{alsa_midi::AlsaMidiClientBackend, MidiClientBackend};
use engine::backend::{pulse::PulseClientBackend, AudioClientBackend};
use engine::midi::parse::MidiParser;
use engine::midi::messages::MidiData;

use synthesizer::{init, one_sample};

//use engine::backend::

fn connect_backend() -> Result<Box<dyn AudioClientBackend>, Box<dyn Error>> {
    let mut backend: Box<dyn AudioClientBackend> = Box::new(PulseClientBackend::new());
    backend.connect()?;

    Ok(backend)
}

fn connect_midi_backend() -> Result<Box<dyn MidiClientBackend>, Box<dyn Error>> {
    let mut backend: Box<dyn MidiClientBackend> = Box::new(AlsaMidiClientBackend::new());
    backend.connect()?;

    Ok(backend)
}

fn write_to_file(output_file: &mut std::fs::File, data: &[f32]) -> Result<(), Box<dyn Error>> {
    let mut data_out = [0_u8; BUFFER_SIZE * 4];

    // TODO: would memcpy work here faster?
    for i in 0..BUFFER_SIZE {
        let num = (data[i] as f32).to_le_bytes();

        data_out[i * 4] = num[0];
        data_out[i * 4 + 1] = num[1];
        data_out[i * 4 + 2] = num[2];
        data_out[i * 4 + 3] = num[3];
    }

    output_file.write_all(&data_out)?;

    Ok(())
}

fn wrapper() -> Result<(), Box<dyn Error>> {
    let mut output_file = std::fs::File::create("audio.raw").unwrap();

    let mut parser = MidiParser::new();

    let backend = connect_backend()?;
    let midi_backend = connect_midi_backend()?;

    let mut buffer_index = 0;
    let mut sample_index = 0;

    let mut state = init();

    loop {
        let midi_in = midi_backend.read().unwrap();
        let mut messages:Vec<MidiData> = Vec::new();

        if !midi_in.is_empty() {
            parser.write_all(midi_in.as_slice())?;

            while !parser.parsed.is_empty() {
                let message = parser.parsed.pop().unwrap();
                messages.push(message);
            }
        }

        let mut buffer = [0_f32; BUFFER_SIZE];

        for sample in buffer.iter_mut() {
            *sample = one_sample(
                &mut state,
                &mut messages,
                sample_index,
            )?;
            sample_index += 1;
        }

        backend.write(&buffer)?;
        write_to_file(&mut output_file, &buffer)?;

        if buffer_index > 3 {
            thread::sleep(Duration::from_millis(
                ((SAMPLE_RATE as u32 / BUFFER_SIZE as u32) / 1000) as u64,
            ));
        }

        buffer_index += 1;
    }
}

fn main() {
    if let Err(error) = wrapper() {
        println!("{:?}", error);
    }
}
