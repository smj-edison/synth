use std::{thread, time::Duration, io::Write};

use engine::constants::{BUFFER_SIZE, SAMPLE_RATE};

use engine::node::{Node, InputType, OutputType};
use engine::node::oscillator::SawOscillatorNode;
use engine::node::envelope::Envelope;
use engine::node::gain::Gain;
use engine::node::filter::{Filter, FilterType};
use engine::backend::{AudioClientBackend, pulse::PulseClientBackend};

//use engine::backend::

fn connect_backend() -> Box<dyn AudioClientBackend> {
    let mut backend:Box<dyn AudioClientBackend> = Box::new(PulseClientBackend::new());
    backend.connect();

    backend
}

fn create_test_envelope() -> Envelope {
    Envelope::new(
        0.01,
        0.3,
        1.0,
        1.0
    )
}

fn create_test_oscillator() -> SawOscillatorNode {
    SawOscillatorNode::new()
}

fn create_test_filter() -> Filter {
    Filter::new(FilterType::Lowpass, 20_000.0, 0.707)
}

fn create_test_gain() -> Gain {
    Gain::new()
}

fn one_sample(envelope: &mut Envelope, osc: &mut SawOscillatorNode, filter: &mut Filter, gain: &mut Gain, gate_value: f64) -> f64 {
    osc.process();

    envelope.receive_audio(InputType::In, osc.get_output_audio(OutputType::Out));
    envelope.receive_audio(InputType::Gate, gate_value);
    envelope.process();

    gain.receive_audio(InputType::In, envelope.get_output_audio(OutputType::Out));
    gain.process();

    filter.receive_audio(InputType::In, gain.get_output_audio(OutputType::Out));
    filter.process();

    filter.get_output_audio(OutputType::Out)
}

fn write_to_file(output_file: &mut std::fs::File, data: &[f64]) {
    let mut data_out = [0_u8; BUFFER_SIZE * 4];

    // TODO: would memcpy work here faster?
    for i in 0..BUFFER_SIZE {
        let num = (data[i] as f32).to_le_bytes();

        data_out[i * 4 + 0] = num[0];
        data_out[i * 4 + 1] = num[1];
        data_out[i * 4 + 2] = num[2];
        data_out[i * 4 + 3] = num[3];
    }

    output_file.write(&data_out);
}

fn main() {
    let mut output_file = std::fs::File::create("audio.raw").unwrap();

    let backend = connect_backend();

    let mut osc = create_test_oscillator();
    let mut envelope = create_test_envelope();
    let mut filter = create_test_filter();
    let mut gain = create_test_gain();

    let mut sample_index = 0;

    let attack_time = 20;

    loop {
        let mut buffer = [0_f64; BUFFER_SIZE];

        for i in 0..BUFFER_SIZE {
            buffer[i] = one_sample(&mut envelope, &mut osc, &mut filter, &mut gain, if sample_index > attack_time { 0.0 } else { 1.0 });
        }

        backend.write(&buffer);
        write_to_file(&mut output_file, &buffer);
        
        if sample_index > 3 {
            thread::sleep(Duration::from_millis(((BUFFER_SIZE as u32) / SAMPLE_RATE as u32).into()));
        }
        
        sample_index += 1;
    }
}
