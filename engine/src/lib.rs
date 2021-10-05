pub mod node;
pub mod backend;

pub mod config {
    pub struct SynthConfig {
        pub samples_per_second: u32
    }
}

pub mod constants {
    pub const PI: f32 = 3.141592653589793;
    pub const TWO_PI: f32 = PI * 2.0;
    pub const BUFFER_SIZE: usize = 512;
}

pub mod util {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
