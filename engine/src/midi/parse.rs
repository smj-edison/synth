use super::messages::*;

use std::io::{Error, Write};

#[derive(Debug)]
pub struct MidiParser {
    buffer: [u8; 512],
    buffer_len: usize,
    expected_message_length: Option<usize>,
    incomplete_message: bool,
    pub parsed: Vec<MidiData>
}

impl MidiParser {
    pub fn new() -> MidiParser {
        MidiParser { 
            buffer: [0_u8; 512],
            buffer_len: 0,
            expected_message_length: Some(0),
            incomplete_message: false,
            parsed: Vec::new()
        }
    }

    fn is_done(&mut self) -> bool {
        if self.buffer_len == 1 { // we got a new message
            match ((self.buffer[0] & 0x0F), self.buffer[0]) {
                // timing clock    start       continue       stop    active sensing   reset
                (_, 0b11111000 | 0b11111010 | 0b11111011 | 0b11111100 | 0b11111110 | 0b11111111) => {
                    self.expected_message_length = Some(0);
                },
                // prog change | chn pressue
                (0b1100 | 0b1101, _) => {
                    self.expected_message_length = Some(2);
                },
                //nt off| nt on  |aftertch|ctrl chg|pitch bend
                (0b1000 | 0b1001 | 0b1010 | 0b1011 | 0b1110, _) => {
                    self.expected_message_length = Some(3);
                },
                // system exclusive TODO: this'll go forever if it's not 0b11110000, like if it's song position
                (0b1111, _) => {
                    self.expected_message_length = None;
                },
                _ => {
                    unimplemented!("Midi parser not fully implemented, received message {:?} (length {})", self.buffer, self.buffer_len);
                }
            };

            if let Some(len) = self.expected_message_length {
                len > 0
            } else {
                false // if none it doesn't know how long it'll be
            }
        } else if Some(self.buffer_len) == self.expected_message_length {
            // reached desired buffer length
            true
        } else if self.expected_message_length == None {
            // custom midi message
            self.buffer[self.buffer_len - 1] == 0b11110111
        } else {
            // message not fully received
            false
        }
    }

    fn process(&mut self) {
        if self.buffer[0] & 0xF0 == 0xF0 {
            // TODO: system message
            return;
        }

        let channel = (self.buffer[0] & 0xF0) >> 4 as Channel;
        let message_type = (self.buffer[0] & 0x0F);

        let parsed_event = match (message_type, self.buffer[0]) {
            // note off | note on
            (0b1000 | 0b1001, _) => {
                let note = self.buffer[1] as Note;
                let velocity = self.buffer[2] as Velocity;

                match message_type {
                    0b1000 => MidiData::NoteOff { channel, note, velocity },
                    0b1001 => MidiData::NoteOn { channel, note, velocity },
                    _ => unreachable!()
                }
            },
            // polyphonic key pressure
            (0b1010, _) => {
                let note = self.buffer[1] as Note;
                let pressure = self.buffer[2] as Pressue;

                MidiData::Aftertouch { channel, note, pressure }
            },
            // control change
            (0b1011, _) => {
                let controller = self.buffer[1] as ControlIndex;
                let value = self.buffer[2] as ControlValue;

                MidiData::ControlChange { channel, controller, value }
            },
            // program change
            (0b1100, _) => {
                let patch = self.buffer[1] as Patch;

                MidiData::ProgramChange { channel, patch }
            },
            // channel aftertouch
            (0b1101, _) => {
                let pressure = self.buffer[1] as Pressue;

                MidiData::ChannelAftertouch { channel, pressure }
            },
            // pitch bend
            (0b1110, _) => {
                let pitch_bend = (self.buffer[1] as u16) | ((self.buffer[2] as u16) << 8) as Bend;

                MidiData::PitchBend { channel, pitch_bend }
            },
            _ => {
                unimplemented!("Midi protocol not fully implemented")
            }
        };

        self.parsed.push(parsed_event);
    }
}

impl Write for MidiParser {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        // add to buffer one by one, seeing if the midi message is complete
        for i in 0..buf.len() {
            self.buffer[self.buffer_len] = buf[i];
            self.buffer_len += 1;

            if self.is_done() {
                self.process();
                self.buffer_len = 0;
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.buffer_len = 0;
        self.incomplete_message = false;

        Ok(())
    }
}
