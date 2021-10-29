use crate::constants::{TWO_PI, SAMPLE_RATE};

use crate::node::{Node, InputType, OutputType};
use crate::wave::tables::{WAVETABLE_SIZE, FREQUENCY_STEPS};
use crate::wave::tables::{SINE_VALUES, SQUARE_VALUES, SAWTOOTH_VALUES, TRIANGLE_VALUES};
use crate::wave::interpolate::interpolate;

pub trait Oscillator {
    fn get_frequency(&self) -> f32;
    fn set_frequency(&mut self, frequency: f32);
}

pub enum Waveform {
    Sine,
    Triangle,
    Sawtooth,
    Square
}

/// A sinsouid oscillator
/// 
/// # Inputs
/// None currently.
///
/// # Outputs
/// `out` - Mono waveform out.
pub struct OscillatorNode {
    phase: f32,
    frequency: f32,
    output_out: f32,
    wavetable: &'static[[f32; WAVETABLE_SIZE]; FREQUENCY_STEPS]
}

impl OscillatorNode {
    pub fn new(waveform: Waveform) -> OscillatorNode {
        OscillatorNode { 
            phase: 0_f32,
            frequency: 440_f32,
            output_out: 0_f32,
            wavetable: match waveform {
                Waveform::Sine => &*SINE_VALUES,
                Waveform::Square => &*SQUARE_VALUES,
                Waveform::Sawtooth => &*SAWTOOTH_VALUES,
                Waveform::Triangle => &*TRIANGLE_VALUES
            }
        }
    }
}

impl Oscillator for OscillatorNode {
    fn get_frequency(&self) -> f32 { self.frequency }
    fn set_frequency(&mut self, frequency: f32) { self.frequency = frequency; }
}

impl Node for OscillatorNode {
    fn process(&mut self) {
        let phase_advance = self.frequency / (SAMPLE_RATE as f32) * TWO_PI;
        self.phase = (self.phase + phase_advance) % TWO_PI;

        self.output_out = interpolate(self.wavetable, self.frequency, self.phase);
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
