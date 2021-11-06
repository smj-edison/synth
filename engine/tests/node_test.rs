use engine::node::{ramp::RampType, AudioNode, Dummy, InputType, OutputType, Ramp};
use simple_error::SimpleError;

use engine::constants::SAMPLE_RATE;

#[test]
fn dummy_node_sets_output_correctly() -> Result<(), SimpleError> {
    let mut dummy_node = Dummy::new();

    // set what value it should output
    dummy_node.set_output_out(0.5);

    // make sure it outputs its set output
    assert!((dummy_node.get_output_audio(OutputType::Out)? - 0.5).abs() < 0.0001);

    Ok(())
}

#[test]
fn dummy_node_sets_receives_input_correctly() -> Result<(), SimpleError> {
    let mut dummy_node = Dummy::new();

    // input audio into it
    dummy_node.receive_audio(InputType::In, 0.3);

    // make sure it saves the audio inputted
    assert!((dummy_node.get_input_in() - 0.3).abs() < 0.0001);

    Ok(())
}

#[test]
fn ramp_node_linear_interpolate_works() -> Result<(), SimpleError> {
    let mut ramp_node = Ramp::new();
    ramp_node.set_ramp_type(RampType::Linear);
    ramp_node.ramp_to_value(2.0, 2.0 / (SAMPLE_RATE as f32));

    // should return 0 the first time
    assert!((ramp_node.get_output_audio(OutputType::Out)? - 0.0).abs() < 0.0001);

    // next 1
    ramp_node.process();
    println!("{}", ramp_node.get_output_audio(OutputType::Out)?);
    assert!((ramp_node.get_output_audio(OutputType::Out)? - 1.0).abs() < 0.0001);

    // and 2
    ramp_node.process();
    assert!((ramp_node.get_output_audio(OutputType::Out)? - 2.0).abs() < 0.0001);

    // shouldn't overshoot
    ramp_node.process();
    assert!((ramp_node.get_output_audio(OutputType::Out)? - 2.0).abs() < 0.0001);

    Ok(())
}

#[test]
fn ramp_node_exponential_interpolate_works() -> Result<(), SimpleError> {
    let mut ramp_node = Ramp::new();

    ramp_node.set_position(110.0);
    ramp_node.process();

    ramp_node.set_ramp_type(RampType::Exponential);
    ramp_node.ramp_to_value(440.0, 2.0 / (SAMPLE_RATE as f32));

    // should return 110 the first time
    assert!((ramp_node.get_output_audio(OutputType::Out)? - 110.0).abs() < 0.0001);

    // next 220
    ramp_node.process();
    println!("{}", ramp_node.get_output_audio(OutputType::Out)?);
    assert!((ramp_node.get_output_audio(OutputType::Out)? - 220.0).abs() < 0.0001);

    // next 440
    ramp_node.process();
    assert!((ramp_node.get_output_audio(OutputType::Out)? - 440.0).abs() < 0.0001);

    // shouldn't overshoot
    ramp_node.process();
    assert!((ramp_node.get_output_audio(OutputType::Out)? - 440.0).abs() < 0.0001);

    Ok(())
}
