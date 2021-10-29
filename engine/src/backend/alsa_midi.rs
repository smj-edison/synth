use std::error::Error;
use std::io::Read;

use simple_error::bail;
use alsa::{Rawmidi, Direction};

use crate::backend::MidiClientBackend;
use crate::midi::MidiMessage;

pub struct AlsaMidiClientBackend {
    client: Option<Rawmidi>,
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
    fn read(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut out = [0_u8; 512];

        let bytes_result = if let Some(client) = &self.client {
            client.io().read(&mut out)
        } else {
            bail!("Midi backend not initialized");
        };

        let bytes_read = match bytes_result {
            Ok(bytes) => bytes,
            Err(error) => {
                if let Some(err) = error.raw_os_error() {
                    if err == -11 {
                        0_usize // there was nothing to read
                    } else {
                        return Err(Box::new(error))
                    }
                } else {
                    return Err(Box::new(error))
                }
            }
        };

        let mut buffer = vec![0; bytes_read];
        
        for i in 0..bytes_read {
            buffer[i] = out[i];
        }

        Ok(buffer)
    }

    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.client = Some(Rawmidi::new(
            "hw:1,0,0",
            Direction::Capture,
            true
        )?);

        Ok(())
    }
}
