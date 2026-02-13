use std::collections::HashMap;
use sal_core::{dbg::Dbg, error::Error};
use crate::{content::Content, db::parameters::ParameterData};
use super::template::Template;


pub struct Parameters {
    dbg: Dbg,
    title: String, 
    table: Template,
}
//
impl Parameters {
    pub fn from(
        parent: &Dbg,
        language: &String,
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, ParameterData>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Template");
        let title = if language.contains("en") {
            "Stability parameters"
        } else {
            "Параметры остойчивости"
        }.to_owned();
        Ok(Self {
            dbg: dbg.clone(),
            title,
            table: Template::from_parameters(
                &dbg,
                language,
                target,
                result,
                ship_wide,
            )?,            
        })
    }
}
//
impl Content for Parameters {
    //
    fn to_string(self) -> Result<String, Error> {
        Ok(format!("### {}\n\n", self.title) + &self.table.to_string()?)
    }
}
