use crate::node::{InputType, AudioNode, OutputType};
use crate::constants::{SAMPLE_RATE};

use simple_error::bail;
use simple_error::SimpleError;

#[derive(Debug)]
pub enum RampType {
    Linear,
    Exponential
}

/// note: exponential ramp cannot use a negative value for from or to!
pub struct Ramp {
    output_out: f32,
    from: f32,
    to: f32,
    at: f32,
    speed: f32,
    duration: f32,
    from_processed: f32, // processed meaning whatever form the ramp type needs the values in for fast calculation
    to_processed: f32,
    ramp_type: RampType
}

impl Ramp {
    pub fn new() -> Ramp {
        Ramp {
            output_out: 0_f32,
            from: 0.0,
            to: 0.0,
            at: 0.0,
            speed: 0.0,
            duration: 0.0,
            from_processed: 0.0,
            to_processed: 0.0,
            ramp_type: RampType::Linear
        }
    }

    /// duration is in seconds
    fn set_ramp_parameters(&mut self, from: f32, to: f32, duration: f32) {
        self.from = from;
        self.to = to;
        self.duration = duration;

        match self.ramp_type {
            RampType::Linear => {
                self.from_processed = self.from;
                self.to_processed = self.to;

                self.at = self.from;
                self.speed = ((self.to - self.from) / self.duration) / SAMPLE_RATE as f32;
            },
            RampType::Exponential => {
                if self.from < 0.0 || self.to < 0.0 {
                    panic!("Cannot use negative values in an exponential ramp");
                    // that is, unless my imaginary number knowledge was better
                }

                // exponential formula in this case is
                // ramp_value = from * 2^at
                // the values that need to be calculated are
                // how far to go linearly to end up in the from-to range exponentially

                self.from_processed = 0.0;
                self.at = 0.0;

                // from * 2^x = to (maximum position)
                // solved is
                // x = log2(to/from)
                // where x is how far to go on the exponential curve before stopping
                if (self.to - self.from).abs() < f32::EPSILON {
                    self.to_processed = 0.0;
                } else {
                    self.to_processed = f32::ln(self.to / self.from) / f32::ln(2.0);
                }

                self.speed = (self.to_processed / self.duration) / SAMPLE_RATE as f32;
            }
        }
    }

    pub fn set_position(&mut self, value: f32) {
        self.ramp_type = RampType::Linear;

        self.from = value;
        self.to = value;
        self.from_processed = value;
        self.to_processed = value;
        self.at = value;

        self.duration = 0.0;
    }

    pub fn get_position(&self) -> f32 {
        match self.ramp_type {
            RampType::Linear => self.at,
            RampType::Exponential => {
                return (self.from * 2_f32.powf(self.at)).clamp(
                    f32::min(self.from, self.to),
                    f32::max(self.from, self.to)
                )
            }
        }
    }

    pub fn set_ramp_type(&mut self, ramp_type: RampType) {
        let from = self.get_position();

        self.ramp_type = ramp_type;

        self.set_ramp_parameters(from, self.to, self.duration);
    }

    pub fn ramp_to_value(&mut self, to: f32, duration: f32) {
        self.set_ramp_parameters(self.get_position(), to, duration);
    }
}

impl Default for Ramp {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioNode for Ramp {
    fn receive_audio(&mut self, input_type: InputType, _input: f32) -> Result<(), SimpleError> {
        bail!("Cannot receive {:?}", input_type);
    }

    fn process(&mut self) {
        self.at += self.speed;
        self.at = self.at.clamp(
            f32::min(self.from_processed, self.to_processed),
            f32::max(self.from_processed, self.to_processed)
        );

        self.output_out = self.get_position();
    }

    fn get_output_audio(&self, output_type: OutputType) -> Result<f32, SimpleError> {
        match output_type {
            OutputType::Out => Ok(self.output_out),
            _ => bail!("Cannot output {:?}", output_type),
        }
    }
}
