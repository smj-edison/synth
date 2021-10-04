use std::collections::HashMap;
use std::{thread, time::Duration};

use engine::constants::BUFFER_SIZE;

use engine::config::SynthConfig;
use engine::node::Node;
use engine::node::oscillator::SawOscillatorNode;
use engine::node::envelope::Envelope;
use engine::backend::{AudioClientBackend, pulse::PulseClientBackend};

//use engine::backend::

fn connect_backend() -> Box<dyn AudioClientBackend> {
    let mut backend:Box<dyn AudioClientBackend> = Box::new(PulseClientBackend::new());
    backend.connect();

    backend
}

fn create_test_envelope() -> Envelope {
    Envelope::new(
        0.2,
        0.3,
        0.5,
        1.0
    )
}

fn create_test_oscillator() -> SawOscillatorNode {
    SawOscillatorNode::new()
}

fn one_sample(envelope: &mut Envelope, osc: &mut SawOscillatorNode, synth_config: &SynthConfig, gate: f32) -> [f32; BUFFER_SIZE] {
    osc.process(&synth_config);

    let mut OOEI = osc.map_outputs(&synth_config); // oscillator out envelope in
    
    OOEI.insert(String::from("gate"), [gate; BUFFER_SIZE]);

    envelope.map_inputs(&OOEI, &synth_config);
    envelope.process(&synth_config);

    let mut arr_out = [0_f32; BUFFER_SIZE];

    arr_out.clone_from(envelope.map_outputs(&synth_config).get(&String::from("out")).unwrap());

    arr_out
}

fn main() {
    let backend = connect_backend();

    let mut osc = create_test_oscillator();
    let mut envelope = create_test_envelope();

    let synth_config = SynthConfig {
        samples_per_second: 48000
    };

    let test = one_sample(&mut envelope, &mut osc, &synth_config, 1.0);

    for i in 0..3 {
        let test = one_sample(&mut envelope, &mut osc, &synth_config, 1.0);
        backend.write(&test);
    }

    loop {
        let test = one_sample(&mut envelope, &mut osc, &synth_config, 1.0);
        backend.write(&test);
    }
}
