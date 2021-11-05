use crate::node::{InputType, AudioNode, OutputType};

use simple_error::bail;
use simple_error::SimpleError;

pub struct Dummy {
    input_in: f32,
    output_out: f32,
}

impl Dummy {
    pub fn new() -> Dummy {
        Dummy {
            input_in: 0_f32,
            output_out: 0_f32,
        }
    }

    pub fn set_output_out(&mut self, output_out: f32) {
        self.output_out = output_out;
    }

    pub fn get_input_in(&self) -> f32 {
        self.input_in
    }
}

impl Default for Dummy {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioNode for Dummy {
    fn receive_audio(&mut self, input_type: InputType, input: f32) -> Result<(), SimpleError> {
        match input_type {
            InputType::In => self.input_in = input,
            _ => bail!("Cannot receive {:?}", input_type),
        }

        Ok(())
    }

    fn process(&mut self) {}

    fn get_output_audio(&self, output_type: OutputType) -> Result<f32, SimpleError> {
        match output_type {
            OutputType::Out => Ok(self.output_out),
            _ => bail!("Cannot output {:?}", output_type),
        }
    }
}
