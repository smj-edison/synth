use std::collections::HashMap;

use engine::constants::BUFFER_SIZE;

use engine::config::SynthConfig;
use engine::node::Node;
use engine::node::oscillator::SinOscillatorNode;

fn main() {
    let mut osc = SinOscillatorNode::new();

    let mut buffers:HashMap<String, [f32; BUFFER_SIZE]> = HashMap::new();

    let synth_config = SynthConfig {
        samples_per_second: 48000
    };


    osc.map_inputs(&buffers, &synth_config);
    osc.process(&synth_config);
    let buffer_out = osc.map_outputs(&synth_config);
    osc.process(&synth_config);
    let buffer_out_2 = osc.map_outputs(&synth_config);

    println!("{:?}", buffer_out);
    println!("{:?}", buffer_out_2);
}
