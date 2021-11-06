use std::io::{stdin, stdout, Write};
use std::error::Error;

use simple_error::bail;
use midir::{MidiInput, Ignore};

use crate::backend::MidiClientBackend;

pub struct MidirMidiClientBackend {
    
}

impl MidirMidiClientBackend {
    pub fn new() -> MidirMidiClientBackend {
        MidirMidiClientBackend { }
    }
}

impl MidiClientBackend for MidirMidiClientBackend {
    fn read(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        // let mut out = [0_u8; 512];

        // let bytes_result = if let Some(client) = &self.client {
        //     client.io().read(&mut out)
        // } else {
        //     bail!("Midi backend not initialized");
        // };

        // let bytes_read = match bytes_result {
        //     Ok(bytes) => bytes,
        //     Err(error) => {
        //         if let Some(err) = error.raw_os_error() {
        //             if err == -11 {
        //                 0_usize // there was nothing to read
        //             } else {
        //                 return Err(Box::new(error));
        //             }
        //         } else {
        //             return Err(Box::new(error));
        //         }
        //     }
        // };
        let bytes_read = 0;

        let mut buffer = vec![0; bytes_read];

        // buffer[..bytes_read].clone_from_slice(&out[..bytes_read]);

        Ok(buffer)
    }

    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();
    
        let mut midi_in = MidiInput::new("midir reading input")?;
        midi_in.ignore(Ignore::None);
        
        // Get an input port (read from console if multiple are available)
        let in_ports = midi_in.ports();
        let in_port = match in_ports.len() {
            0 => return Err("no input port found".into()),
            1 => {
                println!("Choosing the only available input port: {}", midi_in.port_name(&in_ports[0]).unwrap());
                &in_ports[0]
            },
            _ => {
                println!("\nAvailable input ports:");
                for (i, p) in in_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_in.port_name(p).unwrap());
                }
                print!("Please select input port: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                in_ports.get(input.trim().parse::<usize>()?)
                        .ok_or("invalid input port selected")?
            }
        };
        
        println!("\nOpening connection");
        let in_port_name = midi_in.port_name(in_port)?;

        let _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
        }, ())?;
        
        println!("Connection open, reading input from '{}' (press enter to exit) ...", in_port_name);


        Ok(())
    }
}

impl Default for MidirMidiClientBackend {
    fn default() -> MidirMidiClientBackend {
        MidirMidiClientBackend::new()
    }
}
