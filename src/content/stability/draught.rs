use std::collections::HashMap;
use sal_core::{dbg::Dbg, error::Error};
use crate::{content::misc::lang::Lang, content::Content, db::parameters::ParameterData};
use super::template::Template;


pub struct Draught {
    title: String,
    table: Template,
}
//
impl Draught {
    pub fn from(
        parent: &Dbg,
        language: &Lang,
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, ParameterData>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        let title = if *language == Lang::En {
            "Draft parameters"
        } else  {
            "Параметры посадки"
        }.to_owned();
        Ok(Self {
            title,
            table: Template::from_parameters(
                &Dbg::new(parent, "Draught"),
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
    fn to_string(self) -> Result<String, Error> {
        Ok(format!("## {}\n\n", self.title) + &self.table.to_string()?)
    }
}
