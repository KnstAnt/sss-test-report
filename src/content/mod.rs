use crate::error::Error;

pub mod misc;
pub mod strength;
pub mod stability;

//
pub trait Content {
    //
    fn to_string(self) -> Result<String, Error>;
}
