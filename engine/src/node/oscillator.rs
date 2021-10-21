use std::num;

use crate::constants::{PI, TWO_PI, SAMPLE_RATE};

use crate::node::{Node, InputType, OutputType};

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
    output_out: f64
}

impl SinOscillatorNode {
    pub fn new() -> SinOscillatorNode {
        SinOscillatorNode { 
            phase: 0_f64,
            frequency: 440_f64,
            output_out: 0_f64
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

        self.output_out = self.phase.sin();

        self.phase = (self.phase + phase_advance) % TWO_PI;        
    }

    fn receive_audio(&mut self, input_type: InputType, _input: f64) {
        match input_type {
            _ => panic!("Cannot receive {:?}", input_type)
        }
    }

    fn get_output_audio(&self, output_type: OutputType) -> f64 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}


pub struct SawOscillatorNode {
    phase: f64,
    frequency: f64,
    output_out: f64
}

// huge thanks to https://blog.demofox.org/2012/06/18/diy-synth-3-sampling-mixing-and-band-limited-wave-forms/
impl SawOscillatorNode {
    pub fn new() -> SawOscillatorNode {
        SawOscillatorNode { 
            phase: 0_f64,
            frequency: 440_f64,
            output_out: 0_f64
        }
    }
}

impl Oscillator for SawOscillatorNode {
    fn get_frequency(&self) -> f64 { self.frequency }
    fn set_frequency(&mut self, frequency: f64) { self.frequency = frequency; }
}

impl Node for SawOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f64) * TWO_PI;
        self.phase = (self.phase + phase_advance) % TWO_PI;

        let mut num_harmonics = 0;
        
        if self.frequency != 0.0 {
            let mut temp_freq = self.frequency;

            while temp_freq < SAMPLE_RATE as f64 * 0.5 {
                num_harmonics += 1;
                temp_freq += self.frequency;
            }

            //print!("{} ", num_harmonics);
        }

        let mut sin_sum = 0.0;

        for harmonic_index in 1..num_harmonics {
            sin_sum += (self.phase * harmonic_index as f64).sin() / harmonic_index as f64;
        }

        //adjust the volume
        self.output_out = sin_sum * 2.0 / PI;
    }

    fn receive_audio(&mut self, input_type: InputType, _input: f64) {
        match input_type {
            _ => panic!("Cannot receive {:?}", input_type)
        }
    }

    fn get_output_audio(&self, output_type: OutputType) -> f64 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}


pub struct SquareOscillatorNode {
    phase: f64,
    frequency: f64,
    output_out: f64
}

impl SquareOscillatorNode {
    pub fn new() -> SquareOscillatorNode {
        SquareOscillatorNode { 
            phase: 0_f64,
            frequency: 440_f64,
            output_out: 0_f64
        }
    }
}

impl Oscillator for SquareOscillatorNode {
    fn get_frequency(&self) -> f64 { self.frequency }
    fn set_frequency(&mut self, frequency: f64) { self.frequency = frequency; }
}

impl Node for SquareOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f64) * TWO_PI;

        self.output_out = self.phase.sin();

        self.phase = (self.phase + phase_advance) % TWO_PI;

        let mut num_harmonics = 0;
        
        if self.frequency != 0.0 {
            while self.frequency * ((num_harmonics * 2 - 1) as f64) < SAMPLE_RATE as f64 * 0.5 {
                num_harmonics += 1;
            }
        }

        let mut sin_sum = 0.0;

        for harmonic_index in 1..num_harmonics {
            sin_sum += (self.phase * (harmonic_index * 2 - 1) as f64).sin() / ((harmonic_index * 2 - 1) as f64);
        }

        //adjust the volume
        self.output_out = sin_sum * 4.0 / PI;
    }

    fn receive_audio(&mut self, input_type: InputType, _input: f64) {
        match input_type {
            _ => panic!("Cannot receive {:?}", input_type)
        }
    }

    fn get_output_audio(&self, output_type: OutputType) -> f64 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}


pub struct TriangleOscillatorNode {
    phase: f64,
    frequency: f64,
    output_out: f64
}

impl TriangleOscillatorNode {
    pub fn new() -> TriangleOscillatorNode {
        TriangleOscillatorNode { 
            phase: 0_f64,
            frequency: 440_f64,
            output_out: 0_f64
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
        
        self.output_out = if self.phase < 0.5 {
            // phase goes between 0 and 0.5, make it go between 0 - 2 then -1 - 1
            self.phase * 4.0 - 1.0
        } else {
            // phase goes between 0.5 and 1, invert it and make it go between 0 and 0.5, see above for rest
            (1.0 - self.phase) * 4.0 - 1.0
        };

        self.phase = (self.phase + phase_advance) % 1.0;
    }

    fn receive_audio(&mut self, input_type: InputType, _input: f64) {
        match input_type {
            _ => panic!("Cannot receive {:?}", input_type)
        }
    }

    fn get_output_audio(&self, output_type: OutputType) -> f64 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}
