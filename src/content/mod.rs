use sal_core::error::Error;

pub mod displacement;
pub mod misc;
pub mod stability;
pub mod strength;

//
pub trait Content {
    //
    fn to_string(self) -> Result<String, Error>;
}
