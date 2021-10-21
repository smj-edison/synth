use crate::node::{Node, InputType, OutputType};
use crate::constants::{SAMPLE_RATE, PI};

#[derive(Clone, Copy)]
pub enum FilterType {
    Lowpass
}

pub struct Filter {
    filter_type: FilterType,
    frequency: f64,
    q: f64,
    dirty: bool,
    a1: f64,
    a2: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    prev_input_1: f64,
    prev_input_2: f64,
    prev_output_1: f64,
    prev_output_2: f64,
    input_in: f64,
    output_out: f64
}

impl Node for Filter {
    fn receive_audio(&mut self, input_type: InputType, input: f64) {
        self.input_in = input;
    }

    fn process(&mut self) {
        if self.dirty {
            self.recompute();
        }

        let output = 
            (self.b0 * self.input_in) +
            (self.b1 * self.prev_input_1) +
            (self.b2 * self.prev_input_2) -
            (self.a1 * self.prev_output_1) -
            (self.a2 * self.prev_output_2);

        self.prev_input_2 = self.prev_input_1;
        self.prev_input_1 = self.input_in;

        self.prev_output_2 = self.prev_output_1;
        self.prev_output_1 = output;

        self.output_out = output;
    }

    fn get_output_audio(&self, output_type: OutputType) -> f64 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}

impl Filter {
    pub fn new(filter_type: FilterType, frequency: f64, q: f64) -> Filter {
        let mut new_filter = Filter {
            filter_type: filter_type,
            frequency: frequency,
            q: q,
            a1: 0.0,
            a2: 0.0,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            prev_input_1: 0.0,
            prev_input_2: 0.0,
            prev_output_1: 0.0,
            prev_output_2: 0.0,
            input_in: 0_f64,
            output_out: 0_f64,
            dirty: true
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
                let k = (PI * self.frequency / SAMPLE_RATE as f64).tan();
                let norm = 1.0 / (1.0 + k / self.q + k * k);

                b0 = k * k * norm;
                b1 = 2.0 * b0;
                b2 = b0;
                a1 = 2.0 * (k * k - 1.0) * norm;
                a2 = (1.0 - k / self.q + k * k) * norm;
            }
            // FilterType::Lowpass => {
            //     let n = 1.0 / (PI * self.frequency / SAMPLE_RATE as f64);
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

    pub fn get_filter_type(&self) -> FilterType { self.filter_type }
    pub fn set_filter_type(&mut self, filter_type: FilterType) { self.dirty = true; self.filter_type = filter_type; }

    pub fn get_frequency(&self) -> f64 { self.frequency }
    pub fn set_frequency(&mut self, frequency: f64) { self.dirty = true; self.frequency = frequency; }

    pub fn get_q(&self) -> f64 { self.q }
    pub fn set_q(&mut self, q: f64) { self.dirty = true; self.q = q; }
}
