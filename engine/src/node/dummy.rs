use std::collections::HashMap;

use crate::constants::{BUFFER_SIZE};

use crate::node::Node;

pub struct Dummy {
    input_in: [f64; BUFFER_SIZE],
    output_out: [f64; BUFFER_SIZE]
}

impl Dummy {
    pub fn new() -> Dummy {
        Dummy {
            input_in: [0_f64; BUFFER_SIZE],
            output_out: [0_f64; BUFFER_SIZE]
        }
    }

    pub fn set_output_out(&mut self, output_out: [f64; BUFFER_SIZE]) {
        self.output_out = output_out;
    }

    pub fn get_input_in(&self) -> &[f64; BUFFER_SIZE] {
        &self.input_in
    }
}

impl Node for Dummy {
    fn map_inputs(&mut self, buffers: &HashMap<String, [f64; BUFFER_SIZE]>) {
        let buffer_in = match buffers.get(&String::from("out")) {
            Some(gate) => &gate,
            None => &[0_f64; BUFFER_SIZE]
        };

        self.input_in.clone_from(buffer_in);
    }

    fn process(&mut self) {
        
    }

    fn map_outputs(&self) -> HashMap<String, [f64; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f64; BUFFER_SIZE]> = HashMap::new();
        
        outputs.insert(String::from("out"), self.output_out);
        
        //outputs
        outputs
    }
}
