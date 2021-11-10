use simple_error::SimpleError;

use engine::node::envelope::Envelope;
use engine::node::filter::{Filter, FilterType};
use engine::node::gain::Gain;
use engine::node::oscillator::{Oscillator, OscillatorNode, Waveform};
use engine::pipeline::midi_oscillator::MidiOscillator;
use engine::node::ramp::{Ramp, RampType};
use engine::node::{InputType, AudioNode, MidiNode, OutputType};   
use engine::midi::messages::MidiData;

#[allow(dead_code)]
pub struct OneSampleData {
    envelope: Envelope,
    osc: MidiOscillator,
    lfo: OscillatorNode,
    filter: Filter,
    gain: Gain,
    ramp: Ramp,
    notes_on: i32,
}

pub fn init() -> OneSampleData {
    OneSampleData {
        envelope: create_test_envelope(),
        osc: create_test_oscillator(),
        lfo: create_test_lfo(),
        filter: create_test_filter(),
        gain: create_test_gain(),
        ramp: create_test_ramp(),
        notes_on: 0
    }
}

pub fn one_sample(state: &mut OneSampleData, midi: &mut Vec<MidiData>, _sample_index: i32) -> Result<f32, SimpleError> {
    state.osc.receive_midi(InputType::In, midi)?;

    state.osc.process();
    state.lfo.process();

    state.gain.receive_audio(InputType::In, state.osc.get_output_audio(OutputType::Out)?)?;
    state.gain.set_gain(0.2);
    state.gain.process();

    //println!("{}", lfo.get_output_audio(OutputType::Out));

    state.filter.receive_audio(InputType::In, state.gain.get_output_audio(OutputType::Out)?)?;
    state.filter.receive_audio(
        InputType::FilterOffset,
        state.lfo.get_output_audio(OutputType::Out)? * 3.0,
    )?;
    state.filter.process();

    midi.clear();

    state.gain.get_output_audio(OutputType::Out)
}


fn create_test_envelope() -> Envelope {
    Envelope::new(0.01, 0.3, 1.0, 0.3)
}

fn create_test_oscillator() -> MidiOscillator {
    let mut osc = MidiOscillator::new();
    osc.set_waveform(Waveform::Square);
    
    osc
}

fn create_test_lfo() -> OscillatorNode {
    let mut osc = OscillatorNode::new(Waveform::Sine);
    osc.set_frequency(1.0);

    osc
}

fn create_test_filter() -> Filter {
    Filter::new(FilterType::Lowpass, 2_000.0, 0.707)
}

fn create_test_gain() -> Gain {
    Gain::new()
}

fn create_test_ramp() -> Ramp {
    let mut ramp = Ramp::new();
    ramp.set_position(220.0);
    ramp.set_ramp_type(RampType::Exponential);
    ramp.ramp_to_value(880.0, 8.0);

    ramp
}

