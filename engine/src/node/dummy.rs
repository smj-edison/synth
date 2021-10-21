use crate::node::{Node, InputType, OutputType};

pub struct Dummy {
    input_in: f64,
    output_out: f64
}

impl Dummy {
    pub fn new() -> Dummy {
        Dummy {
            input_in: 0_f64,
            output_out: 0_f64
        }
    }

    pub fn set_output_out(&mut self, output_out: f64) {
        self.output_out = output_out;
    }

    pub fn get_input_in(&self) -> f64 {
        self.input_in
    }
}

impl Node for Dummy {
    fn receive_audio(&mut self, input_type: InputType, input: f64) {
        self.input_in = input;
    }

    fn process(&mut self) {
        
    }

    fn get_output_audio(&self, output_type: OutputType) -> f64 {
        self.output_out
    }
}
