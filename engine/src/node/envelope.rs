use crate::config::SynthConfig;
use crate::constants::BUFFER_SIZE;

use crate::node::Node;

use std::collections::HashMap;

pub enum EnvelopeState {
    Attacking,
    Decaying,
    Sustaining,
    Releasing,
    None
}

pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    state: EnvelopeState,
    current_amplitude: f32, // between 0 and 1
    last_gate: bool,
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

                self.current_amplitude += attack_rate;

                if self.current_amplitude >= 1.0 {
                    self.current_amplitude = 1.0;

                    EnvelopeState::Decaying
                } else {
                    EnvelopeState::Attacking
                }                
            }
            Decaying => {
                let decay_rate = (1.0 / config.samples_per_second as f32) / self.decay / (1.0 - self.sustain);

                self.current_amplitude -= decay_rate;

                if self.current_amplitude <= self.sustain {
                    self.current_amplitude = self.sustain;

                    EnvelopeState::Sustaining
                } else {
                    EnvelopeState::Decaying
                }                
            }
            Sustaining => {
                EnvelopeState::Sustaining
            }
            Releasing => {
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

            if(engaged) {
                self.process_gate_engaged();
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
