pub mod dummy;
pub mod envelope;
pub mod filter;
pub mod gain;
pub mod oscillator;

use simple_error::SimpleError;

pub trait AudioNode {
    fn process(&mut self);
    fn receive_audio(&mut self, input_type: InputType, input: f32) -> Result<(), SimpleError>;
    fn get_output_audio(&self, output_type: OutputType) -> Result<f32, SimpleError>;
}

#[derive(Debug)]
pub enum InputType {
    In,
    Gate,
    Detune,
    FilterOffset,
}

#[derive(Debug)]
pub enum OutputType {
    Out,
    None,
}
