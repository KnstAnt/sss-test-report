use std::collections::HashMap;

use crate::{content::Content, db::parameters::ParameterData, error::Error};

use super::template::Template;


pub struct Draught {
    dbg: Dbg,
    title: String,
    table: Template,
}
//
impl Draught {
    pub fn from(
        language: &String,
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, ParameterData>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        let title = if language.contains("en") {
            "Draft parameters"
        } else  {
            "Параметры посадки"
        }.to_owned();
        Ok(Self {
            title,
            table: Template::from_parameters(
                language,
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
        Ok(format!("## {}\n\n", self.title) + &self.table.to_string()?)
    }
}
