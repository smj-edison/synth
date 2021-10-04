use crate::config::SynthConfig;
use crate::constants::BUFFER_SIZE;

use crate::node::Node;

use std::collections::HashMap;

pub enum EnvelopeState {
    Attacking,
    Decaying,
    Sustaining,
    Releasing
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
    amplitude_anchor: f32, // between 0 and 1
    current_amplitude: f32, // between 0 and 1
    buffer_gate: [f32; BUFFER_SIZE],
    buffer_in: [f32; BUFFER_SIZE],
    buffer_out: [f32; BUFFER_SIZE]
}

// TODO: ADSR linear only
impl Envelope {
    fn process_gate_engaged(&mut self, config: &SynthConfig) {
        self.state = match &self.state {
            EnvelopeState::Attacking => {
                let attack_rate = (1.0 / config.samples_per_second as f32) / self.attack;
                self.amplitude_position += attack_rate;

                // take `self.attack` seconds, even if attack started from not complete release
                self.current_amplitude = attack(self.amplitude_anchor, 1.0, self.amplitude_position);

                if self.current_amplitude >= 1.0 {
                    self.current_amplitude = 1.0;
                    self.amplitude_position = 0.0; // reset amplitude position for decay

                    EnvelopeState::Decaying
                } else {
                    EnvelopeState::Attacking
                }                
            }
            EnvelopeState::Decaying => {
                let decay_rate = (1.0 / config.samples_per_second as f32) / self.decay;
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

    fn process_gate_released(&mut self, config: &SynthConfig) {
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
                let release_rate = (1.0 / config.samples_per_second as f32) / self.release;

                self.amplitude_position += release_rate;

                // take `self.attack` seconds, even if attack started from not complete release
                if self.amplitude_position <= 1.0 {
                    self.current_amplitude = release(self.amplitude_anchor, 0.0, self.amplitude_position);
                    self.current_amplitude = self.current_amplitude.clamp(0.0, 1.0);
                }                

                EnvelopeState::Releasing
            }
        }
    }
}

impl Node for Envelope {
    fn map_inputs(&mut self, buffers: &HashMap<String, [f32; BUFFER_SIZE]>, _config: &SynthConfig) {
        let buffer_in = match buffers.get(&String::from("in")) {
            Some(gate) => &gate,
            None => &[0_f32; BUFFER_SIZE]
        };
        
        let buffer_gate = match buffers.get(&String::from("gate")) {
            Some(gate) => &gate,
            None => &[0_f32; BUFFER_SIZE]
        };

        self.buffer_in.clone_from(buffer_in);
        self.buffer_gate.clone_from(buffer_gate);
    }

    fn process(&mut self, config: &SynthConfig) {
        for i in 0..self.buffer_gate.len() {
            let engaged = self.buffer_gate[i] > 0.0;

            if engaged {
                self.process_gate_engaged(config);
            } else {
                self.process_gate_released(config);
            }

            self.buffer_out[i] = self.buffer_in[i] * self.current_amplitude;
        }
    }    

    fn map_outputs(&mut self, _config: &SynthConfig) -> HashMap<String, [f32; BUFFER_SIZE]> {
        let mut outputs:HashMap::<String, [f32; BUFFER_SIZE]> = HashMap::new();

        // TODO: this probably is not efficient
        let buffer_out = std::mem::replace(&mut self.buffer_out, [0_f32; BUFFER_SIZE]);
        
        outputs.insert(String::from("out"), buffer_out);
        
        //outputs
        outputs
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
