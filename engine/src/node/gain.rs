use std::collections::HashMap;

use crate::constants::{BUFFER_SIZE};

use crate::node::Node;

pub struct Gain {
    input_in: [f64; BUFFER_SIZE],
    output_out: [f64; BUFFER_SIZE],
    gain: f64
}

impl Gain {
    pub fn new() -> Gain {
        Gain {
            input_in: [0_f64; BUFFER_SIZE],
            output_out: [0_f64; BUFFER_SIZE],
            gain: 0.1
        }
    }

    pub fn set_buffer_out(&mut self, buffer_out: [f64; BUFFER_SIZE]) {
        self.output_out = buffer_out;
    }

    pub fn get_buffer_in(&self) -> &[f64; BUFFER_SIZE] {
        &self.input_in
    }
}

impl Node for Gain {
    fn map_inputs(&mut self, buffers: &HashMap<String, [f64; BUFFER_SIZE]>) {
        let buffer_in = match buffers.get(&String::from("out")) {
            Some(gate) => &gate,
            None => &[0_f64; BUFFER_SIZE]
        };

        self.input_in.clone_from(buffer_in);
    }

    fn process(&mut self) {
        for i in 0..BUFFER_SIZE {
            let input = self.input_in[i];

            let output = input * self.gain;

            self.output_out[i] = output;
        }
    }

    fn map_outputs(&self) -> HashMap<String, [f64; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f64; BUFFER_SIZE]> = HashMap::new();
        
        outputs.insert(String::from("out"), self.output_out);
        
        //outputs
        outputs
    }
}
