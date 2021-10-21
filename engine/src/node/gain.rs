use crate::node::{Node, InputType, OutputType};

pub struct Gain {
    input_in: f64,
    output_out: f64,
    gain: f64
}

impl Gain {
    pub fn new() -> Gain {
        Gain {
            input_in: 0_f64,
            output_out: 0_f64,
            gain: 0.4
        }
    }
}

impl Node for Gain {
    fn receive_audio(&mut self, input_type: InputType, input: f64) {
        match input_type {
            InputType::In => self.input_in = input,
            _ => panic!("Cannot receive {:?}", input_type)
        }
    }

    fn process(&mut self) {
        let input = self.input_in;

        self.output_out = input * self.gain;
    }

    fn get_output_audio(&self, output_type: OutputType) -> f64 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}
