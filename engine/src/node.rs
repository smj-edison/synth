pub mod oscillator;
pub mod envelope;
pub mod filter;

use std::ptr;

use crate::config::SynthConfig;
use crate::constants::BUFFER_SIZE;

use std::collections::HashMap;

pub trait Node {
    fn process(&mut self);
    fn map_inputs(&mut self, buffers: &HashMap<String, &[f32; BUFFER_SIZE]>);
    fn map_outputs(&self) -> HashMap<String, &[f32; BUFFER_SIZE]>;
}

impl Node {
    pub fn receive<'a>(&mut self, from: &'a Node) -> &'a Node {
        let incoming_buffers = from.map_outputs();

        self.map_inputs(&incoming_buffers);

        from
    }

    pub fn receive_multiple<'a>(&mut self, from_list: Vec<(&'a Node, String, String)>) -> &'a Node {
        let mixed_buffer:HashMap<String, &[f32; BUFFER_SIZE]> = HashMap::new();

        for mapping in from_list {
            let incoming_buffers = mapping.0.map_outputs();

            mixed_buffer.insert(mapping.2, match incoming_buffers.get(&mapping.1) {
                Some(buffer) => buffer,
                None => &[0_f32; BUFFER_SIZE]
            });
        }        

        self.map_inputs(&mixed_buffer);

        from
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
