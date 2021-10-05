use std::collections::HashMap;

use crate::config::SynthConfig;
use crate::constants::{BUFFER_SIZE, TWO_PI, SAMPLE_RATE};

use crate::node::Node;

#[derive(Clone, Copy)]
pub enum FilterType {
    Lowpass
}

pub struct Filter {
    filter_type: FilterType,
    frequency: f32,
    q: f32,
    db_gain: f32,
    dirty: bool,
    a0: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    prev_input_1: f32,
    prev_input_2: f32,
    prev_output_1: f32,
    prev_output_2: f32,
    buffer_in: [f32; BUFFER_SIZE],
    buffer_out: [f32; BUFFER_SIZE]
}

impl Node for Filter {
    fn map_inputs(&mut self, buffers: &HashMap<String, &[f32; BUFFER_SIZE]>) {
        let buffer_in = match buffers.get(&String::from("out")) {
            Some(gate) => &gate,
            None => &[0_f32; BUFFER_SIZE]
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

    fn map_outputs(&self) -> HashMap<String, &[f32; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, &[f32; BUFFER_SIZE]> = HashMap::new();

        // TODO: this probably is not efficient
        let mut buffer_out = [0_f32; BUFFER_SIZE];
        buffer_out.clone_from(&self.buffer_out);
        
        outputs.insert(String::from("out"), &buffer_out);
        
        //outputs
        outputs
    }
}

impl Filter {
    pub fn new(filter_type: FilterType, frequency: f32, q: f32, db_gain: f32, synth_config: &SynthConfig) -> Filter {
        let mut new_filter = Filter {
            filter_type: filter_type,
            frequency: frequency,
            q: q,
            db_gain: db_gain,
            a0: 0.0,
            a1: 0.0,
            a2: 0.0,
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            prev_input_1: 0.0,
            prev_input_2: 0.0,
            prev_output_1: 0.0,
            prev_output_2: 0.0,
            buffer_in: [0_f32; BUFFER_SIZE],
            buffer_out: [0_f32; BUFFER_SIZE],
            dirty: true
        };

        new_filter.recompute();

        new_filter
    }

    fn recompute(&mut self,) {
        let a = 10_f32.powf(self.db_gain / 40.0);
        let omega = TWO_PI * self.frequency / SAMPLE_RATE as f32;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * self.q);
        let beta = (a * a).sqrt();

        let mut a0;
        let mut a1;
        let mut a2;
        let mut b0;
        let mut b1;
        let mut b2;

        match &self.filter_type {
            FilterType::Lowpass => {
                b0 = (1.0 - cs) / 2.0;
                b1 = 1.0 - cs;
                b2 = (1.0 - cs) / 2.0;
                a0 = 1.0 + alpha;
                a1 = -2.0 * cs;
                a2 = 1.0 - alpha;
            }
        };
        
        a1 /= a0;
        a2 /= a0;
        b0 /= a0;
        b1 /= a0;
        b2 /= a0;

        self.a0 = a0;
        self.a1 = a1;
        self.a2 = a2;
        self.b0 = b0;
        self.b1 = b1;
        self.b2 = b2;

        self.dirty = false;
    }

    fn get_filter_type(&self) -> FilterType { self.filter_type }
    fn set_filter_type(&mut self, filter_type: FilterType) { self.dirty = true; self.filter_type = filter_type; }

    fn get_frequency(&self) -> f32 { self.frequency }
    fn set_frequency(&mut self, frequency: f32) { self.dirty = true; self.frequency = frequency; }

    fn get_q(&self) -> f32 { self.q }
    fn set_q(&mut self, q: f32) { self.dirty = true; self.q = q; }

    fn get_db_gain(&self) -> f32 { self.db_gain }
    fn set_db_gain(&mut self, db_gain: f32) { self.dirty = true; self.db_gain = db_gain; }
}
