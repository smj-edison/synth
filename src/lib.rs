use simple_error::SimpleError;

use engine::constants::{BUFFER_SIZE, SAMPLE_RATE};

use engine::node::envelope::Envelope;
use engine::node::filter::{Filter, FilterType};
use engine::node::gain::Gain;
use engine::node::oscillator::{Oscillator, OscillatorNode, Waveform};
use engine::pipeline::ramped_oscillator::RampedOscillator;
use engine::node::ramp::{Ramp, RampType};
use engine::node::{InputType, AudioNode, OutputType};   
use engine::midi::messages::MidiData;

pub struct OneSampleData {
    envelope: Envelope,
    osc: RampedOscillator,
    lfo: OscillatorNode,
    filter: Filter,
    gain: Gain,
    ramp: Ramp,
    gate_value: f32,
}

pub fn init() -> OneSampleData {
    OneSampleData {
        envelope: create_test_envelope(),
        osc: create_test_oscillator(),
        lfo: create_test_lfo(),
        filter: create_test_filter(),
        gain: create_test_gain(),
        ramp: create_test_ramp(),
        gate_value: 0.0
    }
}

pub fn one_sample(data: &mut OneSampleData, midi: &mut Vec<MidiData>, sample_index: i32) -> Result<f32, SimpleError> {
    data.osc.process();
    data.ramp.process();
    //osc.set_frequency(ramp.get_output_audio(OutputType::Out)?);

    //osc.set_frequency(lfo.get_output_audio(OutputType::Out) * 500.0 + 700.0);

    data.lfo.process();

    data.envelope.receive_audio(InputType::Gate, data.gate_value)?;
    data.envelope.process();

    data.gain.receive_audio(InputType::In, data.osc.get_output_audio(OutputType::Out)?)?;
    data.gain.set_gain(/*envelope.get_output_audio(OutputType::Out)? * */0.1);
    data.gain.process();

    //println!("{}", lfo.get_output_audio(OutputType::Out));

    data.filter.receive_audio(InputType::In, data.gain.get_output_audio(OutputType::Out)?)?;
    data.filter.receive_audio(
        InputType::FilterOffset,
        data.lfo.get_output_audio(OutputType::Out)? * 3.0,
    )?;
    data.filter.process();

    midi.clear();

    data.gain.get_output_audio(OutputType::Out)
}


fn create_test_envelope() -> Envelope {
    Envelope::new(0.01, 0.3, 1.0, 1.0)
}

fn create_test_oscillator() -> RampedOscillator {
    let mut osc = RampedOscillator::new();
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

