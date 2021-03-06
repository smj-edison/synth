#![allow(clippy::needless_range_loop)]

#[macro_use]
extern crate lazy_static;
extern crate simple_error;

pub mod backend;
pub mod midi;
pub mod node;
pub mod util;
pub mod wave;
pub mod pipeline;

pub mod config {
    pub struct SynthConfig {
        pub samples_per_second: u32,
    }
}

pub mod constants {
    #[allow(clippy::excessive_precision)]
    pub const PI: f32 = 3.14159265358979323846264338327950288f32;
    pub const TWO_PI: f32 = PI * 2.0;
    pub const BUFFER_SIZE: usize = 512;
    pub const SAMPLE_RATE: u32 = 48_000;
}
