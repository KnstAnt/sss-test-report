use sal_core::error::Error;

pub mod displacement;
pub mod misc;
pub mod stability;
pub mod strength;

//
pub trait Content {
    //
    fn table(self) -> Result<String, Error>;
    //
    fn title(&self) -> String;
    //
    fn to_string(self) -> Result<String, Error> where Self: Sized {
        Ok(format!("{}\n\n{}", self.title(), self.table()?))
    }
}
