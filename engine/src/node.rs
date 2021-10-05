pub mod oscillator;
pub mod envelope;
pub mod filter;

use std::ptr;

use crate::config::SynthConfig;
use crate::constants::BUFFER_SIZE;

use std::collections::HashMap;

pub trait Node {
    fn process(&mut self, config: &SynthConfig);
    fn map_inputs(&mut self, buffers: &HashMap<String, [f32; BUFFER_SIZE]>, config: &SynthConfig);
    fn map_outputs(&mut self, config: &SynthConfig) -> HashMap<String, [f32; BUFFER_SIZE]>;
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
