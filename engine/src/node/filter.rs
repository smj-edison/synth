use std::collections::HashMap;

use crate::constants::{BUFFER_SIZE, PI, SAMPLE_RATE};

use crate::node::Node;

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
    buffer_in: [f64; BUFFER_SIZE],
    buffer_out: [f64; BUFFER_SIZE]
}

impl Node for Filter {
    fn map_inputs(&mut self, buffers: &HashMap<String, [f64; BUFFER_SIZE]>) {
        let buffer_in = match buffers.get(&String::from("out")) {
            Some(gate) => &gate,
            None => &[0_f64; BUFFER_SIZE]
        };

        self.buffer_in.clone_from(buffer_in);
    }

    fn process(&mut self) {
        if self.dirty {
            self.recompute();
        }

        for i in 0..BUFFER_SIZE {
            let input = self.buffer_in[i];

            let output = 
                (self.b0 * input) +
                (self.b1 * self.prev_input_1) +
                (self.b2 * self.prev_input_2) -
                (self.a1 * self.prev_output_1) -
                (self.a2 * self.prev_output_2);

            self.prev_input_2 = self.prev_input_1;
            self.prev_input_1 = input;

            self.prev_output_2 = self.prev_output_1;
            self.prev_output_1 = output;

            self.buffer_out[i] = output;
        }
    }

    fn map_outputs(&self) -> HashMap<String, [f64; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f64; BUFFER_SIZE]> = HashMap::new();
        
        outputs.insert(String::from("out"), self.buffer_out);
        
        //outputs
        outputs
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
            buffer_in: [0_f64; BUFFER_SIZE],
            buffer_out: [0_f64; BUFFER_SIZE],
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
