use std::collections::HashMap;

use crate::constants::{BUFFER_SIZE};

use crate::node::Node;

pub struct Dummy {
    buffer_in: [f64; BUFFER_SIZE],
    buffer_out: [f64; BUFFER_SIZE]
}

impl Dummy {
    pub fn new() -> Dummy {
        Dummy {
            buffer_in: [0_f64; BUFFER_SIZE],
            buffer_out: [0_f64; BUFFER_SIZE]
        }
    }

    pub fn set_buffer_out(&mut self, buffer_out: [f64; BUFFER_SIZE]) {
        self.buffer_out = buffer_out;
    }

    pub fn get_buffer_in(&self) -> &[f64; BUFFER_SIZE] {
        &self.buffer_in
    }
}

impl Node for Dummy {
    fn map_inputs(&mut self, buffers: &HashMap<String, [f64; BUFFER_SIZE]>) {
        let buffer_in = match buffers.get(&String::from("out")) {
            Some(gate) => &gate,
            None => &[0_f64; BUFFER_SIZE]
        };

        self.buffer_in.clone_from(buffer_in);
    }

    fn process(&mut self) {
        
    }

    fn map_outputs(&self) -> HashMap<String, [f64; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f64; BUFFER_SIZE]> = HashMap::new();
        
        outputs.insert(String::from("out"), self.buffer_out);
        
        //outputs
        outputs
    }
}
