use std::collections::HashMap;

use crate::{content::{stability::template::Template, Content}, error::Error};

pub struct Summary {
    table: Template,
}
//
impl Summary {
    pub fn from(
        language: &String, 
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, f64>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        Ok(Self {
            table: Template::from(
                language,
                target,
                result,
                ship_wide,
            )?,            
        })
    }
}
//
impl Content for Summary {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}
