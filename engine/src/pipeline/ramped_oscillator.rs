use crate::node::oscillator::Oscillator;
use crate::node::{AudioNode, InputType, OutputType, Ramp, ramp::RampType, OscillatorNode, oscillator::Waveform};

use simple_error::bail;
use simple_error::SimpleError;

pub struct RampedOscillator {
    output_out: f32,
    frequency_ramp: Ramp,
    portamento: f32,
    oscillator: OscillatorNode
}

impl RampedOscillator {
    pub fn new() -> RampedOscillator {
        RampedOscillator {
            output_out: 0_f32,
            portamento: 0.1,
            frequency_ramp: Ramp::new_with_start_value(440.0),
            oscillator: OscillatorNode::new_with_frequency(Waveform::Sine, 440.0)
        }
    }

    pub fn set_portamento(&mut self, portamento: f32) {
        self.portamento = portamento;
    }

    pub fn get_portamento(&self) -> f32 {
        self.portamento
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency_ramp.set_ramp_type(RampType::Exponential);
        self.frequency_ramp.ramp_to_value(frequency, self.portamento);
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency_ramp.get_position()
    }

    pub fn set_frequency_hard(&mut self, frequency: f32) {
        self.frequency_ramp.set_position(frequency);
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.oscillator.set_waveform(waveform);
    }
}

impl AudioNode for RampedOscillator {
    fn receive_audio(&mut self, input_type: InputType, _input: f32) -> Result<(), SimpleError> {
        bail!("Cannot receive {:?}", input_type);
    }

    fn process(&mut self) {
        self.frequency_ramp.process();

        self.oscillator.set_frequency(self.frequency_ramp.get_output_audio(OutputType::Out).unwrap());
        self.oscillator.process();

        self.output_out = self.oscillator.get_output_audio(OutputType::Out).unwrap();
    }

    fn get_output_audio(&self, output_type: OutputType) -> Result<f32, SimpleError> {
        match output_type {
            OutputType::Out => Ok(self.output_out),
            _ => bail!("Cannot output {:?}", output_type),
        }
    }
}

impl Default for RampedOscillator {
    fn default() -> Self {
        Self::new()
    }
}
