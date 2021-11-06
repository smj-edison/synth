use crate::node::{AudioNode, MidiNode, InputType, OutputType};

use simple_error::bail;
use simple_error::SimpleError;

use crate::node::{Envelope, Gain, oscillator::Waveform};
use crate::pipeline::ramped_oscillator::RampedOscillator;
use crate::midi::messages::MidiData;

pub struct MidiOscillator {
    midi_in: Vec<MidiData>,
    envelope: Envelope,
    osc: RampedOscillator,
    gain: Gain,
    output_out: f32,
    notes_on: i32
}

impl MidiOscillator {
    pub fn new() -> MidiOscillator {
        MidiOscillator {
            midi_in: Vec::new(),
            envelope: Envelope::new(0.01, 0.2, 1.0, 0.3),
            osc: RampedOscillator::new(),
            gain: Gain::new(),
            output_out: 0.0,
            notes_on: 0
        }
    }

    pub fn set_portamento(&mut self, portamento: f32) {
        self.osc.set_portamento(portamento);
    }

    pub fn get_portamento(&self) -> f32 {
        self.osc.get_portamento()
    }

    pub fn get_adsr(&self) -> (f32, f32, f32, f32) {
        self.envelope.get_adsr()
    }

    pub fn set_adsr(&mut self, attack: f32, decay: f32, sustain: f32, release: f32) {
        self.envelope.set_adsr(attack, decay, sustain, release);
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.osc.set_waveform(waveform);
    }
}

impl AudioNode for MidiOscillator {
    fn receive_audio(&mut self, input_type: InputType, _input: f32) -> Result<(), SimpleError> {
        bail!("Cannot receive {:?}", input_type);
    }

    fn process(&mut self) {
        if !self.midi_in.is_empty() {
            for message in self.midi_in.iter_mut() {
                match message {
                    MidiData::NoteOn {channel, note, velocity} => {
                        self.notes_on += 1;
                        self.osc.set_frequency(440.0 * f32::powf(2.0, (*note as f32 - 69.0) / 12.0));
                    }
                    MidiData::NoteOff {channel, note, velocity} => {
                        self.notes_on -= 1;
                    }
                    _ => {}
                }
            }
        }

        self.osc.process();

        self.envelope.receive_audio(InputType::Gate, if self.notes_on > 0 {1.0} else {0.0}).unwrap();
        self.envelope.process();

        self.gain.receive_audio(InputType::In, self.osc.get_output_audio(OutputType::Out).unwrap()).unwrap();
        self.gain.set_gain(self.envelope.get_output_audio(OutputType::Out).unwrap());
        self.gain.process();

        self.output_out = self.gain.get_output_audio(OutputType::Out).unwrap();
    }

    fn get_output_audio(&self, output_type: OutputType) -> Result<f32, SimpleError> {
        match output_type {
            OutputType::Out => Ok(self.output_out),
            _ => bail!("Cannot output {:?}", output_type),
        }
    }
}

impl MidiNode for MidiOscillator {
    fn receive_midi(&mut self, input_type: InputType, input: &Vec<MidiData>) -> Result<(), SimpleError> {
        match input_type {
            InputType::In => self.midi_in.clone_from(input),
            _ => bail!("Cannot receive {:?}", input_type),
        }

        Ok(())
    }
}
