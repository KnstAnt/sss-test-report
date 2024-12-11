//pub mod general;
//pub mod list_of_calculations;
pub mod table;
pub mod displacement;
pub mod chart_strength;

//
pub trait Content {
    //
    fn to_string(self) -> String;
}
