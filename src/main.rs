use std::{thread, time::Duration};

use engine::constants::BUFFER_SIZE;

use engine::config::SynthConfig;
use engine::node::Node;
use engine::node::oscillator::SawOscillatorNode;
use engine::node::envelope::Envelope;
use engine::node::dummy::Dummy;
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
    Filter::new(FilterType::Lowpass, 1000.0, 5.0, 1.0)
}

fn one_sample(envelope: &mut Envelope, osc: &mut SawOscillatorNode, filter: &mut Filter, synth_config: &SynthConfig, gate: f32) -> [f32; BUFFER_SIZE] {
    // osc.process();

    let gate_sample = [gate; BUFFER_SIZE];
    let mut gate = Dummy::new();

    gate.set_buffer_out(gate_sample);

    // let mut OOEI = osc.map_outputs(); // oscillator out envelope in
    
    // OOEI.insert(String::from("gate"), &[gate; BUFFER_SIZE]);

    // envelope.map_inputs(&OOEI);
    // envelope.process();

    // let EOFI = envelope.map_outputs(); // envelope out filter in

    // filter.map_inputs(&EOFI);
    // filter.process();

    // let mut arr_out = [0_f32; BUFFER_SIZE];

    // arr_out.clone_from(filter.map_outputs().get(&String::from("out")).unwrap());

    osc.process();

    // arr_out
    let envelope = envelope as &mut dyn Node;
    let filter = filter as &mut dyn Node;
    let gate = &mut gate as &mut dyn Node;

    envelope.receive_multiple(vec![
        (osc, String::from("out"), String::from("out")),
        (gate, String::from("out"), String::from("gate"))
    ]);

    envelope.process();

    filter.receive(envelope);

    filter.process();

    *filter.map_outputs().get(&String::from("out")).unwrap()
}

fn main() {
    let backend = connect_backend();

    let synth_config = SynthConfig {
        samples_per_second: 48000
    };

    let mut osc = create_test_oscillator();
    let mut envelope = create_test_envelope();
    let mut filter = create_test_filter();

    let mut current_sample = 0;

    for i in 0..3 {
        let test = one_sample(&mut envelope, &mut osc, &mut filter, &synth_config, 1.0);
        backend.write(&test);

        current_sample += 1;
    }

    loop {
        let test = one_sample(&mut envelope, &mut osc, &mut filter, &synth_config, 0.0);
        backend.write(&test);
        
        thread::sleep(Duration::from_millis(((BUFFER_SIZE as u32) / synth_config.samples_per_second).into()));

        current_sample += 1;
    }
}
