use crate::constants::{BUFFER_SIZE, TWO_PI, SAMPLE_RATE};

use crate::node::Node;

use std::collections::HashMap;

pub trait Oscillator {
    fn get_frequency(&self) -> f64;
    fn set_frequency(&mut self, frequency: f64);
}

/// A sinsouid oscillator
/// 
/// # Inputs
/// None currently.
///
/// # Outputs
/// `out` - Mono waveform out.
pub struct SinOscillatorNode {
    phase: f64,
    frequency: f64,
    output_out: [f64; BUFFER_SIZE]
}

impl SinOscillatorNode {
    pub fn new() -> SinOscillatorNode {
        SinOscillatorNode { 
            phase: 0_f64,
            frequency: 440_f64,
            output_out: [0_f64; BUFFER_SIZE]
        }
    }
}

impl Oscillator for SinOscillatorNode {
    fn get_frequency(&self) -> f64 { self.frequency }
    fn set_frequency(&mut self, frequency: f64) { self.frequency = frequency; }
}

impl Node for SinOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f64) * TWO_PI;

        let mut buffer_out = [0_f64; BUFFER_SIZE];

        for i in 0..BUFFER_SIZE {
            buffer_out[i] = self.phase.sin();
            self.phase = (self.phase + phase_advance) % TWO_PI;
        }

        self.output_out = buffer_out;
    }

    fn map_inputs(&mut self, _buffers: &HashMap<String, [f64; BUFFER_SIZE]>) {
        // Nothing to do, perhaps detune in the future?
    }

    fn map_outputs(&self) -> HashMap<String, [f64; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f64; BUFFER_SIZE]> = HashMap::new();

        // TODO: this probably is not efficient
        let mut buffer_out = [0_f64; BUFFER_SIZE];
        buffer_out.clone_from(&self.output_out);
        
        outputs.insert(String::from("out"), buffer_out);
        
        //outputs
        outputs
    }
}


pub struct SawOscillatorNode {
    phase: f64,
    frequency: f64,
    output_out: [f64; BUFFER_SIZE]
}

impl SawOscillatorNode {
    pub fn new() -> SawOscillatorNode {
        SawOscillatorNode { 
            phase: 0_f64,
            frequency: 440_f64,
            output_out: [0_f64; BUFFER_SIZE]
        }
    }
}

impl Oscillator for SawOscillatorNode {
    fn get_frequency(&self) -> f64 { self.frequency }
    fn set_frequency(&mut self, frequency: f64) { self.frequency = frequency; }
}

impl Node for SawOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f64);

        let mut buffer_out = [0_f64; BUFFER_SIZE];

        for i in 0..BUFFER_SIZE {
            buffer_out[i] = self.phase % 1.0;
            self.phase = (self.phase + phase_advance) % 1.0;
        }

        self.output_out = buffer_out;
    }

    fn map_inputs(&mut self, _buffers: &HashMap<String, [f64; BUFFER_SIZE]>) {
        // Nothing to do, perhaps detune in the future?
    }

    fn map_outputs(&self) -> HashMap<String, [f64; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f64; BUFFER_SIZE]> = HashMap::new();
        
        outputs.insert(String::from("out"), self.output_out);
        
        //outputs
        outputs
    }
}


pub struct SquareOscillatorNode {
    phase: f64,
    frequency: f64,
    output_out: [f64; BUFFER_SIZE],
    duty_cycle: f64
}

impl SquareOscillatorNode {
    pub fn new() -> SquareOscillatorNode {
        SquareOscillatorNode { 
            phase: 0_f64,
            frequency: 440_f64,
            output_out: [0_f64; BUFFER_SIZE],
            duty_cycle: 0.5
        }
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: f64) { self.duty_cycle = duty_cycle; }
    pub fn get_duty_cycle(&mut self) -> f64 { self.duty_cycle }
}

impl Oscillator for SquareOscillatorNode {
    fn get_frequency(&self) -> f64 { self.frequency }
    fn set_frequency(&mut self, frequency: f64) { self.frequency = frequency; }
}

impl Node for SquareOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f64);

        let mut buffer_out = [0_f64; BUFFER_SIZE];

        for i in 0..BUFFER_SIZE {
            buffer_out[i] = if self.phase >= self.duty_cycle { 1.0 } else { -1.0 };
            self.phase = (self.phase + phase_advance) % 1.0;
        }

        self.output_out = buffer_out;
    }

    fn map_inputs(&mut self, _buffers: &HashMap<String, [f64; BUFFER_SIZE]>) {
        // Nothing to do, perhaps detune in the future?
    }

    fn map_outputs(&self) -> HashMap<String, [f64; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f64; BUFFER_SIZE]> = HashMap::new();
        
        outputs.insert(String::from("out"), self.output_out);
        
        //outputs
        outputs
    }
}


pub struct TriangleOscillatorNode {
    phase: f64,
    frequency: f64,
    output_out: [f64; BUFFER_SIZE]
}

impl TriangleOscillatorNode {
    pub fn new() -> TriangleOscillatorNode {
        TriangleOscillatorNode { 
            phase: 0_f64,
            frequency: 440_f64,
            output_out: [0_f64; BUFFER_SIZE]
        }
    }
}

impl Oscillator for TriangleOscillatorNode {
    fn get_frequency(&self) -> f64 { self.frequency }
    fn set_frequency(&mut self, frequency: f64) { self.frequency = frequency; }
}

impl Node for TriangleOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f64);

        let mut buffer_out = [0_f64; BUFFER_SIZE];

        for i in 0..BUFFER_SIZE {
            buffer_out[i] = if self.phase < 0.5 {
                // phase goes between 0 and 0.5, make it go between 0 - 2 then -1 - 1
                self.phase * 4.0 - 1.0
            } else {
                // phase goes between 0.5 and 1, invert it and make it go between 0 and 0.5, see above for rest
                (1.0 - self.phase) * 4.0 - 1.0
            };

            self.phase = (self.phase + phase_advance) % 1.0;
        }

        self.output_out = buffer_out;
    }

    fn map_inputs(&mut self, _buffers: &HashMap<String, [f64; BUFFER_SIZE]>) {
        // Nothing to do, perhaps detune in the future?
    }

    fn map_outputs(&self) -> HashMap<String, [f64; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f64; BUFFER_SIZE]> = HashMap::new();
        
        outputs.insert(String::from("out"), self.output_out);
        
        //outputs
        outputs
    }
}
