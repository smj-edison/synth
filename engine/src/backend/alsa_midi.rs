use std::error::Error;
use std::ffi::CString;

use alsa::{Seq, Direction};
use alsa::seq::{PortCap, PortType, Event};

use crate::backend::MidiClientBackend;
use crate::midi::MidiMessage;

pub struct AlsaMidiClientBackend {
    client: Option<Seq>,
    in_port: Option<i32>
}

impl AlsaMidiClientBackend {
    pub fn new() -> AlsaMidiClientBackend {
        AlsaMidiClientBackend {
            client: None, in_port: None
        }
    }
}

impl MidiClientBackend for AlsaMidiClientBackend {
    fn read(&self) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        let midi_in: Vec<MidiMessage> = vec![];

        let input = self.client.unwrap().input();

        if input.event_input_pending(false)? == 0 {
            return Ok(None)
        } else {
            while input.event_input_pending(false)? > 0 {
                input.event_input()?.get_data()
            }
        }

        Ok(None)
    }

    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.client = Some(Seq::open(None, Some(Direction::Capture), true)?);
        self.client.unwrap().set_client_name(CString::new("Midi Listener").unwrap().as_c_str());
        self.in_port = Some(self.client.unwrap().create_simple_port(
            CString::new("listen:in").unwrap().as_c_str(),
            PortCap::WRITE|PortCap::SUBS_WRITE,
            PortType::APPLICATION
        )?);

        Ok(())
    }

    fn drain(&self) -> Result<(), Box<dyn Error>> {

    }
}
