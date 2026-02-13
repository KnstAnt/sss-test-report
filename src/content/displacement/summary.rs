use crate::{
    content::{stability::template::Template, Content},
    db::parameters::ParameterData,
};
use sal_core::{dbg::Dbg, error::Error};
use std::collections::HashMap;

pub struct Summary {
    dbg: Dbg,
    table: Template,
}
//
impl Summary {
    pub fn from(
        parent: &Dbg,
        language: &String,
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, ParameterData>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Summary");
        Ok(Self {
            dbg: dbg.clone(),
            table: Template::from_parameters(&dbg, language, target, result, ship_wide)?,
        })
    }
}
//
impl Content for Summary {
    //
    fn to_string(self) -> Result<String, Error> {
        self.table.to_string()
    }
}
