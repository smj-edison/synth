use crate::constants::SAMPLE_RATE;

use crate::node::{InputType, AudioNode, OutputType};

pub enum EnvelopeState {
    Attacking,
    Decaying,
    Sustaining,
    Releasing,
}

pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    state: EnvelopeState,
    amplitude_position: f32, // between 0 and 1
    // amplitude_anchor is the spot where the attack is being based on
    // if the note was pressed down again before a complete release, it should attack
    // based on the current amplitude, not jump to 0
    amplitude_anchor: f32,  // between 0 and 1
    current_amplitude: f32, // between 0 and 1
    input_gate: f32,
    input_in: f32,
    output_out: f32,
}

// TODO: ADSR linear only
impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Envelope {
        Envelope {
            attack,
            decay,
            sustain,
            release,
            state: EnvelopeState::Releasing,
            amplitude_position: 0.0,
            amplitude_anchor: 0.0,
            current_amplitude: 0.0,
            input_gate: 0_f32,
            input_in: 0_f32,
            output_out: 0_f32,
        }
    }

    fn process_gate_engaged(&mut self) {
        self.state = match &self.state {
            EnvelopeState::Attacking => {
                let attack_rate = (1.0 / SAMPLE_RATE as f32) / self.attack;
                self.amplitude_position += attack_rate;

                // take `self.attack` seconds, even if attack started from not complete release
                self.current_amplitude =
                    attack(self.amplitude_anchor, 1.0, self.amplitude_position);

                if self.current_amplitude >= 1.0 {
                    self.current_amplitude = 1.0;
                    self.amplitude_position = 0.0; // reset amplitude position for decay

                    EnvelopeState::Decaying
                } else {
                    EnvelopeState::Attacking
                }
            }
            EnvelopeState::Decaying => {
                let decay_rate = (1.0 / SAMPLE_RATE as f32) / self.decay;
                self.amplitude_position += decay_rate;

                self.current_amplitude = decay(1.0, self.sustain, self.amplitude_position);

                if self.current_amplitude <= self.sustain {
                    self.current_amplitude = self.sustain;
                    self.amplitude_position = 0.0; // reset amplitude position for release

                    EnvelopeState::Sustaining
                } else {
                    EnvelopeState::Decaying
                }
            }
            EnvelopeState::Sustaining => {
                self.current_amplitude = self.sustain;

                EnvelopeState::Sustaining
            }
            EnvelopeState::Releasing => {
                self.amplitude_position = 0.0;
                self.amplitude_anchor = self.current_amplitude;

                EnvelopeState::Attacking
            }
        }
    }

    fn process_gate_released(&mut self) {
        self.state = match &self.state {
            EnvelopeState::Attacking => {
                // must have been released, as state is attacking and gate is off
                self.amplitude_position = 0.0;
                self.amplitude_anchor = self.current_amplitude;

                EnvelopeState::Releasing
            }
            EnvelopeState::Decaying => {
                self.amplitude_position = 0.0;
                self.amplitude_anchor = self.current_amplitude;

                EnvelopeState::Releasing
            }
            EnvelopeState::Sustaining => {
                self.amplitude_position = 0.0;
                self.amplitude_anchor = self.current_amplitude;

                EnvelopeState::Releasing
            }
            EnvelopeState::Releasing => {
                let release_rate = (1.0 / SAMPLE_RATE as f32) / self.release;

                self.amplitude_position += release_rate;

                // take `self.attack` seconds, even if attack started from not complete release
                if self.amplitude_position <= 1.0 {
                    self.current_amplitude =
                        release(self.amplitude_anchor, 0.0, self.amplitude_position);
                    self.current_amplitude = self.current_amplitude.clamp(0.0, 1.0);
                }

                EnvelopeState::Releasing
            }
        }
    }
}

impl AudioNode for Envelope {
    fn receive_audio(&mut self, input_type: InputType, input: f32) {
        match input_type {
            InputType::In => self.input_in = input,
            InputType::Gate => self.input_gate = input,
            _ => panic!("Cannot receive {:?}", input_type),
        }
    }

    fn process(&mut self) {
        let engaged = self.input_gate > 0.0;

        if engaged {
            self.process_gate_engaged();
        } else {
            self.process_gate_released();
        }

        self.output_out = self.input_in * self.current_amplitude;
    }

    fn get_output_audio(&self, output_type: OutputType) -> f32 {
        match output_type {
            OutputType::Out => self.output_out,
            _ => panic!("Cannot output {:?}", output_type),
        }
    }
}

fn attack(start: f32, end: f32, amount: f32) -> f32 {
    lerp(start, end, amount)
}

fn decay(start: f32, end: f32, amount: f32) -> f32 {
    lerp(start, end, amount)
}

fn release(start: f32, end: f32, amount: f32) -> f32 {
    lerp(start, end, amount)
}

fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    (end - start) * amount + start
}
