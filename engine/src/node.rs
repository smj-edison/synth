pub mod oscillator;
pub mod envelope;
pub mod filter;
pub mod dummy;
pub mod gain;

use std::ptr;

pub trait Node {
    fn process(&mut self);
    fn receive_audio(&mut self, input_type: InputType, input: f32);
    fn get_output_audio(&self, output_type: OutputType) -> f32;
}

#[derive(Debug)]
pub enum InputType {
    In, Gate, Detune, FilterOffset
}

#[derive(Debug)]
pub enum OutputType {
    Out, None
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
