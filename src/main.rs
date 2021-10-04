use std::collections::HashMap;
use std::{thread, time::Duration};

use engine::constants::BUFFER_SIZE;

use engine::config::SynthConfig;
use engine::node::Node;
use engine::node::oscillator::SinOscillatorNode;
use engine::backend::{AudioClientBackend, pulse::PulseClientBackend};

//use engine::backend::

fn main() {
    let mut backend:Box<dyn AudioClientBackend> = Box::new(PulseClientBackend::new());
    backend.connect();

    let mut osc = SinOscillatorNode::new();

    let buffers:HashMap<String, [f32; BUFFER_SIZE]> = HashMap::new();
    let silence = [0_f32; BUFFER_SIZE];

    let synth_config = SynthConfig {
        samples_per_second: 48000
    };

    //osc.map_inputs(&buffers, &synth_config);
    osc.process(&synth_config);
    let buffer_out = osc.map_outputs(&synth_config);

    println!("{:?}", buffer_out);

    for i in 0..3 {
        osc.process(&synth_config);
        let mut buffer_out = osc.map_outputs(&synth_config);

        backend.write(buffer_out.entry(String::from("out")).or_insert(silence));
    }

    loop {
        osc.process(&synth_config);
        let mut buffer_out = osc.map_outputs(&synth_config);

        backend.write(buffer_out.entry(String::from("out")).or_insert(silence));

        thread::sleep(Duration::from_millis(((BUFFER_SIZE as u32) / synth_config.samples_per_second).into()));
    }
}
