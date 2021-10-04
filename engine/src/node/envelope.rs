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
            Attacking => {
                let attack_rate = (1.0 / config.samples_per_second as f32) / self.attack;

                self.amplitude_position += attack_rate;
                // take `self.attack` seconds, even if attack started from not complete release
                self.current_amplitude = attack(self.amplitude_anchor, 1.0, self.amplitude_position);

                if self.current_amplitude >= 1.0 {
                    self.current_amplitude = 1.0;
                    self.amplitude_position = 0; // reset amplitude position for decay

                    EnvelopeState::Decaying
                } else {
                    EnvelopeState::Attacking
                }                
            }
            Decaying => {
                let attack_rate = (1.0 / config.samples_per_second as f32) / self.attack;

                self.current_amplitude -= decay_rate;

                if self.current_amplitude <= self.sustain {
                    self.current_amplitude = self.sustain;

                    EnvelopeState::Sustaining
                } else {
                    EnvelopeState::Decaying
                }                
            }
            Sustaining => {
                self.current_amplitude = self.sustain;

                EnvelopeState::Sustaining
            }
            Releasing => {
                self.amplitude_position = inv_attack(self.current_amplitude.clamp(0.0, 1.0), 0.0, 1.0);
                self.amplitude_anchor = self.current_amplitude;

                EnvelopeState::Attacking
            }
        }
    }

    fn process_gate_released(&mut self, config: &SynthConfig) {
        self.state = match &self.state {
            Attacking => {
                EnvelopeState::Releasing
            }
            Decaying => {
                EnvelopeState::Releasing              
            }
            Sustaining => {
                EnvelopeState::Releasing
            }
            Releasing => {
                let release_rate = (1.0 / config.samples_per_second as f32) / self.decay / (self.sustain);

                EnvelopeState::Releasing
            }
        }
    }
}

impl Node for Envelope {
    fn map_inputs(&mut self, buffers: &HashMap<String, [f32; BUFFER_SIZE]>, config: &SynthConfig) {
        let mut buffer_in = match buffers.get(&String::from("in")) {
            Some(gate) => &gate,
            None => &[0_f32; BUFFER_SIZE]
        };
        
        let mut buffer_gate = match buffers.get(&String::from("gate")) {
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

    fn map_outputs(&mut self, config: &SynthConfig) -> HashMap<String, [f32; BUFFER_SIZE]> {
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

fn inv_attack(value: f32, start: f32, end: f32) -> f32 {
    inv_lerp(value, start, end)
}

fn decay(start: f32, end: f32, amount: f32) -> f32 {
    lerp(start, end, amount)
}


fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    (end - start) * amount + start
}

fn inv_lerp(value: f32, start: f32, end: f32) -> f32 {
    // TODO: it's a possibility that start = end, which would result in a panic
    (value - start) / (end - start)
}

