use simple_error::bail;
use simple_error::SimpleError;

use crate::node::{AudioNode, InputType, OutputType};

pub struct Gain {
    input_in: f32,
    output_out: f32,
    gain: f32,
}

impl Gain {
    pub fn new() -> Gain {
        Gain {
            input_in: 0_f32,
            output_out: 0_f32,
            gain: 0.4,
        }
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    pub fn get_gain(&self) -> f32 {
        self.gain
    }
}

impl AudioNode for Gain {
    fn receive_audio(&mut self, input_type: InputType, input: f32) -> Result<(), SimpleError> {
        match input_type {
            InputType::In => self.input_in = input,
            _ => bail!("Cannot receive {:?}", input_type),
        }

        Ok(())
    }

    fn process(&mut self) {
        let input = self.input_in;

        self.output_out = input * self.gain;
    }

    fn get_output_audio(&self, output_type: OutputType) -> Result<f32, SimpleError> {
        match output_type {
            OutputType::Out => Ok(self.output_out),
            _ => bail!("Cannot output {:?}", output_type),
        }
    }
}

impl Default for Gain {
    fn default() -> Gain {
        Gain::new()
    }
}
