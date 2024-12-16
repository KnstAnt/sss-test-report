use std::collections::HashMap;

use crate::{content::Content, error::Error};

use super::template::Template;


pub struct Draught {
    table: Template,
}
//
impl Draught {
    pub fn from_data(
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, f64>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        Ok(Self {
            table: Template::from_string(
                target,
                result,
                ship_wide,
            )?,            
        })
    }
}
//
impl Content for Draught {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        Ok("## Параметры посадки\n".to_string() + &self.table.to_string()?)
    }
}
