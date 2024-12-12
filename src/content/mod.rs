use crate::error::Error;

//pub mod general;
//pub mod list_of_calculations;
pub mod misc;
pub mod displacement;
pub mod draught;
pub mod strength;
pub mod stability;

//
pub trait Content {
    //
    fn to_string(self) -> Result<String, Error>;
}
