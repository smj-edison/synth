use std::{thread, time::Duration};

use engine::constants::{BUFFER_SIZE, SAMPLE_RATE};

use engine::config::SynthConfig;
use engine::node::Node;
use engine::node::oscillator::SinOscillatorNode;
use engine::node::envelope::Envelope;
use engine::node::dummy::Dummy;
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

fn create_test_oscillator() -> SinOscillatorNode {
    SinOscillatorNode::new()
}

fn create_test_filter() -> Filter {
    Filter::new(FilterType::Lowpass, 100000.0, 0.707)
}

fn create_test_gain() -> Gain {
    Gain::new()
}

fn one_sample(envelope: &mut Envelope, osc: &mut SinOscillatorNode, filter: &mut Filter, gain: &mut Gain, gate: f64) -> [f64; BUFFER_SIZE] {
    let gate_sample = [gate; BUFFER_SIZE];
    let mut gate = Dummy::new();

    gate.set_buffer_out(gate_sample);
    
    let envelope = envelope as &mut dyn Node;
    let filter = filter as &mut dyn Node;
    let gain = gain as &mut dyn Node;
    let gate = &mut gate as &mut dyn Node;

    // arr_out
    osc.process();
    envelope.receive_multiple_and_process(vec![
        (osc, String::from("out"), String::from("out")),
        (gate, String::from("out"), String::from("gate"))
    ]);
    gain.receive_and_process(osc);
    filter.receive_and_process(gain);
    

    *filter.map_outputs().get(&String::from("out")).unwrap()
}

fn main() {
    let backend = connect_backend();

    let mut osc = create_test_oscillator();
    let mut envelope = create_test_envelope();
    let mut filter = create_test_filter();
    let mut gain = create_test_gain();

    let mut current_sample = 0;

    let attack_time = 50;

    for i in 0..3 {
        let test = one_sample(&mut envelope, &mut osc, &mut filter, &mut gain, if current_sample > attack_time { 0.0 } else { 1.0 });
        backend.write(&test);

        current_sample += 1;
    }

    loop {
        let test = one_sample(&mut envelope, &mut osc, &mut filter, &mut gain, if current_sample > attack_time { 0.0 } else { 1.0 });
        backend.write(&test);
        
        thread::sleep(Duration::from_millis(((BUFFER_SIZE as u32) / SAMPLE_RATE as u32).into()));

        current_sample += 1;
    }
}
