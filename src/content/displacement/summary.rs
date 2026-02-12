use std::collections::HashMap;
use sal_core::{dbg::Dbg, error::Error};
use crate::{content::{stability::template::Template, Content}, db::parameters::ParameterData};

pub struct Summary {
    table: Template,
}
//
impl Summary {
    pub fn from(
        language: &String, 
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, ParameterData>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        Ok(Self {
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
impl Content for Summary {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}
