pub mod pulse;

use std::error::Error;

pub trait AudioClientBackend {
    fn write(&self, data: &[f64]) -> Result<(), Box<dyn Error>>;
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn drain(&self) -> Result<(), Box<dyn Error>>;
}
