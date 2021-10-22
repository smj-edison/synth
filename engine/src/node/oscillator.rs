use crate::constants::{PI, TWO_PI, SAMPLE_RATE};

use crate::node::{Node, InputType, OutputType};
use crate::util::fast_sin_norm_phase;

pub trait Oscillator {
    fn get_frequency(&self) -> f32;
    fn set_frequency(&mut self, frequency: f32);
}

/// A sinsouid oscillator
/// 
/// # Inputs
/// None currently.
///
/// # Outputs
/// `out` - Mono waveform out.
pub struct SinOscillatorNode {
    phase: f32,
    frequency: f32,
    output_out: f32
}

impl SinOscillatorNode {
    pub fn new() -> SinOscillatorNode {
        SinOscillatorNode { 
            phase: 0_f32,
            frequency: 440_f32,
            output_out: 0_f32
        }
    }
}

impl Oscillator for SinOscillatorNode {
    fn get_frequency(&self) -> f32 { self.frequency }
    fn set_frequency(&mut self, frequency: f32) { self.frequency = frequency; }
}

impl Node for SinOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f32) * TWO_PI;

        self.output_out = self.phase.sin();

        self.phase = (self.phase + phase_advance) % TWO_PI;        
    }

    fn receive_audio(&mut self, input_type: InputType, _input: f32) {
        match input_type {
            _ => panic!("Cannot receive {:?}", input_type)
        }
    }

    fn get_output_audio(&self, output_type: OutputType) -> f32 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}


pub struct SawOscillatorNode {
    phase: f32,
    frequency: f32,
    output_out: f32
}

// huge thanks to https://blog.demofox.org/2012/06/18/diy-synth-3-sampling-mixing-and-band-limited-wave-forms/
impl SawOscillatorNode {
    pub fn new() -> SawOscillatorNode {
        SawOscillatorNode { 
            phase: 0_f32,
            frequency: 440_f32,
            output_out: 0_f32
        }
    }
}

impl Oscillator for SawOscillatorNode {
    fn get_frequency(&self) -> f32 { self.frequency }
    fn set_frequency(&mut self, frequency: f32) { self.frequency = frequency; }
}

impl Node for SawOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f32);
        self.phase = (self.phase + phase_advance) % 1.0;

        let mut num_harmonics = 0;
        
        if self.frequency != 0.0 {
            let mut temp_freq = self.frequency;

            while temp_freq < SAMPLE_RATE as f32 * 0.5 {
                num_harmonics += 1;
                temp_freq += self.frequency;
            }
        }

        let mut sin_sum = 0.0;

        for harmonic_index in 1..num_harmonics {
            sin_sum += fast_sin_norm_phase(self.phase * harmonic_index as f32) / harmonic_index as f32;
        }

        //adjust the volume
        self.output_out = sin_sum * 2.0 / PI;
    }

    fn receive_audio(&mut self, input_type: InputType, _input: f32) {
        match input_type {
            _ => panic!("Cannot receive {:?}", input_type)
        }
    }

    fn get_output_audio(&self, output_type: OutputType) -> f32 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}


pub struct SquareOscillatorNode {
    phase: f32,
    frequency: f32,
    output_out: f32
}

impl SquareOscillatorNode {
    pub fn new() -> SquareOscillatorNode {
        SquareOscillatorNode { 
            phase: 0_f32,
            frequency: 440_f32,
            output_out: 0_f32
        }
    }
}

impl Oscillator for SquareOscillatorNode {
    fn get_frequency(&self) -> f32 { self.frequency }
    fn set_frequency(&mut self, frequency: f32) { self.frequency = frequency; }
}

impl Node for SquareOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f32) * TWO_PI;
        self.phase = (self.phase + phase_advance) % TWO_PI;

        let mut num_harmonics = 0;
        
        if self.frequency != 0.0 {
            while self.frequency * ((num_harmonics * 2 - 1) as f32) < SAMPLE_RATE as f32 * 0.5 {
                num_harmonics += 1;
            }
        }

        let mut sin_sum = 0.0;

        for harmonic_index in 1..num_harmonics {
            sin_sum += (self.phase * (harmonic_index * 2 - 1) as f32).sin() / ((harmonic_index * 2 - 1) as f32);
        }

        //adjust the volume
        self.output_out = sin_sum * 4.0 / PI;
    }

    fn receive_audio(&mut self, input_type: InputType, _input: f32) {
        match input_type {
            _ => panic!("Cannot receive {:?}", input_type)
        }
    }

    fn get_output_audio(&self, output_type: OutputType) -> f32 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}


pub struct TriangleOscillatorNode {
    phase: f32,
    frequency: f32,
    output_out: f32
}

impl TriangleOscillatorNode {
    pub fn new() -> TriangleOscillatorNode {
        TriangleOscillatorNode { 
            phase: 0_f32,
            frequency: 440_f32,
            output_out: 0_f32
        }
    }
}

impl Oscillator for TriangleOscillatorNode {
    fn get_frequency(&self) -> f32 { self.frequency }
    fn set_frequency(&mut self, frequency: f32) { self.frequency = frequency; }
}

impl Node for TriangleOscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f32) * TWO_PI;
        self.phase = (self.phase + phase_advance) % TWO_PI;

        let mut num_harmonics = 0;
        
        if self.frequency != 0.0 {
            while self.frequency * ((num_harmonics * 2 - 1) as f32) < SAMPLE_RATE as f32 * 0.5 {
                num_harmonics += 1;
            }
        }

        let mut sin_sum = 0.0;
        let mut subtract = false;

        for harmonic_index in 1..num_harmonics {
            if subtract {
                sin_sum -= (self.phase * (harmonic_index * 2 - 1) as f32).sin() / ((harmonic_index * 2 - 1) * (harmonic_index * 2 - 1)) as f32;
            } else {
                sin_sum += (self.phase * (harmonic_index * 2 - 1) as f32).sin() / ((harmonic_index * 2 - 1) * (harmonic_index * 2 - 1)) as f32;
            }

            subtract = !subtract;
        }

        //adjust the volume
        self.output_out = sin_sum * 4.0 / PI;
    }

    fn receive_audio(&mut self, input_type: InputType, _input: f32) {
        match input_type {
            _ => panic!("Cannot receive {:?}", input_type)
        }
    }

    fn get_output_audio(&self, output_type: OutputType) -> f32 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type)
        }
    }
}
