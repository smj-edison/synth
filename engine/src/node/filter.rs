use simple_error::SimpleError;
use simple_error::bail;

use crate::constants::{PI, SAMPLE_RATE};
use crate::node::{InputType, AudioNode, OutputType};

#[derive(Clone, Copy)]
pub enum FilterType {
    Lowpass,
}

pub struct Filter {
    filter_type: FilterType,
    frequency: f32,
    q: f32,
    dirty: bool,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    prev_input_1: f32,
    prev_input_2: f32,
    prev_output_1: f32,
    prev_output_2: f32,
    filter_offset_in: f32,
    input_in: f32,
    output_out: f32,
}

impl AudioNode for Filter {
    fn receive_audio(&mut self, input_type: InputType, input: f32) -> Result<(), SimpleError> {
        match input_type {
            InputType::FilterOffset => self.filter_offset_in = input,
            InputType::In => self.input_in = input,
            _ => bail!("Cannot receive {:?}", input_type),
        }

        Ok(())
    }

    fn process(&mut self) {
        self.recompute();

        let output = (self.b0 * self.input_in)
            + (self.b1 * self.prev_input_1)
            + (self.b2 * self.prev_input_2)
            - (self.a1 * self.prev_output_1)
            - (self.a2 * self.prev_output_2);

        self.prev_input_2 = self.prev_input_1;
        self.prev_input_1 = self.input_in;

        self.prev_output_2 = self.prev_output_1;
        self.prev_output_1 = output;

        self.output_out = output;
    }

    fn get_output_audio(&self, output_type: OutputType) -> Result<f32, SimpleError> {
        match output_type {
            OutputType::Out => Ok(self.output_out),
            _ => bail!("Cannot output {:?}", output_type),
        }
    }
}

impl Filter {
    pub fn new(filter_type: FilterType, frequency: f32, q: f32) -> Filter {
        let mut new_filter = Filter {
            filter_type,
            frequency,
            q,
            a1: 0.0,
            a2: 0.0,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            prev_input_1: 0.0,
            prev_input_2: 0.0,
            prev_output_1: 0.0,
            prev_output_2: 0.0,
            filter_offset_in: 0.0,
            input_in: 0_f32,
            output_out: 0_f32,
            dirty: true,
        };

        new_filter.recompute();

        new_filter
    }

    fn recompute(&mut self) {
        let a1;
        let a2;
        let b0;
        let b1;
        let b2;

        match &self.filter_type {
            FilterType::Lowpass => {
                let freq = (self.frequency + (self.filter_offset_in * 10_000.0))
                    .clamp(0.0, SAMPLE_RATE as f32 * 0.5);
                //println!("{}", freq);

                let k = (PI * freq / SAMPLE_RATE as f32).tan();
                let norm = 1.0 / (1.0 + k / self.q + k * k);

                b0 = k * k * norm;
                b1 = 2.0 * b0;
                b2 = b0;
                a1 = 2.0 * (k * k - 1.0) * norm;
                a2 = (1.0 - k / self.q + k * k) * norm;
            } // FilterType::Lowpass => {
              //     // clamp to prevent the filter becoming unstable
              //     let freq = (self.frequency + (self.filter_offset_in * 10_000.0)).clamp(1.0, SAMPLE_RATE as f32 * 0.5);

              //     let n = 1.0 / (PI * freq / SAMPLE_RATE as f32);
              //     let n_squared = n * n;
              //     let inv_q = 1.0 / self.q;
              //     let c1 = 1.0 / (1.0 + inv_q * n + n_squared);

              //     b0 = c1;
              //     b1 = c1 * 2.0;
              //     b2 = c1;

              //     a1 = c1 * 2.0 * (1.0 - n_squared);
              //     a2 = c1 * (1.0 - inv_q * n + n_squared);
              // }
        };

        self.a1 = a1;
        self.a2 = a2;
        self.b0 = b0;
        self.b1 = b1;
        self.b2 = b2;

        self.dirty = false;
    }

    pub fn get_filter_type(&self) -> FilterType {
        self.filter_type
    }
    pub fn set_filter_type(&mut self, filter_type: FilterType) {
        self.dirty = true;
        self.filter_type = filter_type;
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }
    pub fn set_frequency(&mut self, frequency: f32) {
        self.dirty = true;
        self.frequency = frequency;
    }

    pub fn get_q(&self) -> f32 {
        self.q
    }
    pub fn set_q(&mut self, q: f32) {
        self.dirty = true;
        self.q = q;
    }
}
