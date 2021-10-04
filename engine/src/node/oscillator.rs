use crate::config::SynthConfig;
use crate::constants::{BUFFER_SIZE, TWO_PI};

use crate::node::Node;

use std::collections::HashMap;

pub trait Oscillator {
    fn get_frequency(&self) -> f32;
    fn set_frequency(&mut self, frequency: f32);
}

pub struct SinOscillatorNode {
    phase: f32,
    frequency: f32,
    buffer_out: [f32; BUFFER_SIZE]
}

impl SinOscillatorNode {
    pub fn new() -> SinOscillatorNode {
        SinOscillatorNode { 
            phase: 0_f32,
            frequency: 440_f32,
            buffer_out: [0_f32; BUFFER_SIZE]
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

impl Oscillator for SinOscillatorNode {
    fn get_frequency(&self) -> f32 { self.frequency }
    fn set_frequency(&mut self, frequency: f32) { self.frequency = frequency; }
}

impl Node for SinOscillatorNode {
    fn process(&mut self, config: &SynthConfig) {
        let phase_advance = self.frequency / (config.samples_per_second as f32) * TWO_PI;

        let mut buffer_out = [0_f32; BUFFER_SIZE];

        for i in 0..BUFFER_SIZE {
            buffer_out[i] = self.phase.sin();
            self.phase = (self.phase + phase_advance) % TWO_PI;
        }

        self.buffer_out = buffer_out;
    }

    fn map_inputs(&mut self, buffers: &HashMap<String, [f32; BUFFER_SIZE]>, config: &SynthConfig) {
        // Nothing to do, perhaps detune in the future?
    }

    fn map_outputs(&mut self, config: &SynthConfig) -> HashMap<String, [f32; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f32; BUFFER_SIZE]> = HashMap::new();

        // TODO: this probably is not efficient
        let buffer_out = std::mem::replace(&mut self.buffer_out, [0_f32; BUFFER_SIZE]);
        
        outputs.insert(String::from("out"), buffer_out);
        
        //outputs
        outputs
    }
}
