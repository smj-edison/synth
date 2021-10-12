pub mod oscillator;
pub mod envelope;
pub mod filter;
pub mod dummy;
pub mod gain;

use std::ptr;

use crate::constants::BUFFER_SIZE;

use std::collections::HashMap;

pub trait Node {
    fn process(&mut self);
    fn map_inputs(&mut self, buffers: &HashMap<String, [f64; BUFFER_SIZE]>);
    fn map_outputs(&self) -> HashMap<String, [f64; BUFFER_SIZE]>;
}

impl dyn Node {
    pub fn receive(&mut self, from: &dyn Node) {
        let incoming_buffers = from.map_outputs();

        self.map_inputs(&incoming_buffers);
        self.process();
    }

    pub fn receive_multiple<'a>(&mut self, from_list: Vec<(&'a dyn Node, String, String)>) {
        let mut mixed_buffer:HashMap<String, [f64; BUFFER_SIZE]> = HashMap::new();

        for mapping in from_list {
            let incoming_buffers = mapping.0.map_outputs();

            let mut buffer_copied = [0_f64; BUFFER_SIZE];

            let buffer = match incoming_buffers.get(&mapping.1) {
                Some(buffer) => buffer,
                None => &[0_f64; BUFFER_SIZE]
            };

            buffer_copied.clone_from(buffer);

            mixed_buffer.insert(mapping.2, buffer_copied);
        }        

        self.map_inputs(&mixed_buffer);
        self.process();
    }

    pub fn receive_and_process(&mut self, from: &dyn Node) {
        self.receive(from);
        self.process();
    }

    pub fn receive_multiple_and_process<'a>(&mut self, from_list: Vec<(&'a dyn Node, String, String)>) {
        self.receive_multiple(from_list);
        self.process();
    }
}

// Do a pointer compare to see if two nodes are the same
impl PartialEq for dyn Node {
    fn eq(&self, other: &dyn Node) -> bool {
        ptr::eq(self, other)
    }

    fn ne(&self, other: &dyn Node) -> bool {
        !ptr::eq(self, other)
    }
}
