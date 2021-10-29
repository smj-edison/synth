#![allow(clippy::needless_range_loop)]

use std::error::Error;
use std::{io::Write, thread, time::Duration};

use engine::constants::{BUFFER_SIZE, SAMPLE_RATE};

use engine::backend::{alsa_midi::AlsaMidiClientBackend, MidiClientBackend};
use engine::backend::{pulse::PulseClientBackend, AudioClientBackend};
use engine::node::envelope::Envelope;
use engine::node::filter::{Filter, FilterType};
use engine::node::gain::Gain;
use engine::node::oscillator::{Oscillator, OscillatorNode, Waveform};
use engine::node::{InputType, Node, OutputType};

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

fn create_test_envelope() -> Envelope {
    Envelope::new(0.01, 0.3, 1.0, 1.0)
}

fn create_test_oscillator() -> OscillatorNode {
    OscillatorNode::new(Waveform::Square)
}

fn create_test_lfo() -> OscillatorNode {
    let mut osc = OscillatorNode::new(Waveform::Sine);
    osc.set_frequency(1.0);

    osc
}

fn create_test_filter() -> Filter {
    Filter::new(FilterType::Lowpass, 10_000.0, 0.707)
}

fn create_test_gain() -> Gain {
    Gain::new()
}

fn one_sample(
    envelope: &mut Envelope,
    osc: &mut OscillatorNode,
    lfo: &mut OscillatorNode,
    filter: &mut Filter,
    gain: &mut Gain,
    gate_value: f32,
    _sample_index: i32,
) -> f32 {
    osc.process();

    //osc.set_frequency(lfo.get_output_audio(OutputType::Out) * 500.0 + 700.0);

    lfo.process();

    envelope.receive_audio(InputType::In, osc.get_output_audio(OutputType::Out));
    envelope.receive_audio(InputType::Gate, gate_value);
    envelope.process();

    gain.receive_audio(InputType::In, osc.get_output_audio(OutputType::Out));
    gain.process();

    //println!("{}", lfo.get_output_audio(OutputType::Out));

    filter.receive_audio(InputType::In, gain.get_output_audio(OutputType::Out));
    filter.receive_audio(
        InputType::FilterOffset,
        lfo.get_output_audio(OutputType::Out),
    );
    filter.process();

    filter.get_output_audio(OutputType::Out)
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

    let backend = connect_backend()?;
    let midi_backend = connect_midi_backend()?;

    let mut osc = create_test_oscillator();
    let mut lfo = create_test_lfo();
    let mut envelope = create_test_envelope();
    let mut filter = create_test_filter();
    let mut gain = create_test_gain();

    let mut buffer_index = 0;
    let mut sample_index = 0;

    let attack_time = 20;

    loop {
        let midi_in = midi_backend.read().unwrap();

        if !midi_in.is_empty() {
            println!("{:?}", midi_in);
        }

        let mut buffer = [0_f32; BUFFER_SIZE];

        for sample in buffer.iter_mut().take(BUFFER_SIZE) {
            *sample = one_sample(
                &mut envelope,
                &mut osc,
                &mut lfo,
                &mut filter,
                &mut gain,
                if buffer_index > attack_time { 0.0 } else { 1.0 },
                sample_index,
            );
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
