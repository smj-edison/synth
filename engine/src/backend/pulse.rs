use std::error::Error;

use pulse::sample::Spec;
use pulse::stream::Direction;
use psimple::Simple;

use crate::backend::AudioClientBackend;

pub struct PulseClientBackend {
    pub pulse_spec: Option<pulse::sample::Spec>,
    pub client: Option<psimple::Simple>
}

impl PulseClientBackend {
    pub fn new() -> PulseClientBackend {
        PulseClientBackend {
            pulse_spec: None, client: None
        }
    }
}

impl AudioClientBackend for PulseClientBackend {
    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let spec = Spec {
            format: pulse::sample::Format::S16NE,
            channels: 2,
            rate: 48_000, // FIXME: This should be passed in
        };
        assert!(spec.is_valid());

        let s = Simple::new(
            None,                // Use the default server
            "Synthesizer Test",  // Our application’s name
            Direction::Playback, // We want a playback stream
            None,                // Use the default device
            "Music",             // Description of our stream
            &spec,               // Our sample format
            None,                // Use default channel map
            None                 // Use default buffering attributes
        )?;

        self.pulse_spec = Some(spec);
        self.client = Some(s);

        Ok(())
    }

    fn write(&self, data: &[f32]) -> Result<(), Box<dyn Error>> {
        let mapped:Box<[u8]> = data.iter().map(|&x| (x.min(1_f32).max(-1_f32) * 127.0 + 127.0) as u8).collect();

        match &self.client {
            Some(client) => client.write(&*mapped),
            None => unimplemented!(),
        }?;

        Ok(())
    }

    fn drain(&self) -> Result<(), Box<dyn Error>> {
        match &self.client {
            Some(client) => client.drain(),
            None => unimplemented!(),
        }?;

        Ok(())
    }
}
