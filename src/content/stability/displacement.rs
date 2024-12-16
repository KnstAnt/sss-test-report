use std::collections::HashMap;

use crate::{content::Content, error::Error};

use super::template::Template;

pub struct Displacement {
    table: Template,
}
//
impl Displacement {
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
impl Content for Displacement {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        Ok("## Водоизмещение  \n".to_string() + &self.table.to_string()?)
    }
}
