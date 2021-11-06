use engine::node::{AudioNode, Dummy, OutputType, InputType};
use simple_error::SimpleError;

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
