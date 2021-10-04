use std::collections::HashMap;
use std::{thread, time::Duration};

use engine::constants::BUFFER_SIZE;

use engine::config::SynthConfig;
use engine::node::Node;
use engine::node::oscillator::TriangleOscillatorNode;
use engine::node::envelope::Envelope;
use engine::backend::{AudioClientBackend, pulse::PulseClientBackend};

//use engine::backend::

fn main() {
    let gate = 
        [1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];

    // let mut backend:Box<dyn AudioClientBackend> = Box::new(PulseClientBackend::new());
    // backend.connect();

    let mut envelope = Envelope::new(
        150.0 / 48000.0,
        50.0 / 48000.0,
        0.8,
        100.0 / 48000.0
    );

    let mut noise = [1_f32; 512];

    let mut buffers:HashMap<String, [f32; BUFFER_SIZE]> = HashMap::new();

    buffers.insert(String::from("out"), [1_f32; BUFFER_SIZE]);
    buffers.insert(String::from("gate"), gate);

    let synth_config = SynthConfig {
        samples_per_second: 48000
    };

    envelope.map_inputs(&buffers, &synth_config);
    envelope.process(&synth_config);
    let buffer_out = envelope.map_outputs(&synth_config);

    println!("{:?}", buffer_out);

    // for i in 0..3 {
    //     osc.process(&synth_config);
    //     let mut buffer_out = osc.map_outputs(&synth_config);

    //     backend.write(buffer_out.entry(String::from("out")).or_insert(silence));
    // }

    // loop {
    //     osc.process(&synth_config);
    //     let mut buffer_out = osc.map_outputs(&synth_config);

    //     backend.write(buffer_out.entry(String::from("out")).or_insert(silence));

    //     thread::sleep(Duration::from_millis(((BUFFER_SIZE as u32) / synth_config.samples_per_second).into()));
    // }
}
