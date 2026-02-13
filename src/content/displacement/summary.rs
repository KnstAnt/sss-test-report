use crate::{
    content::{Content, misc::lang::Lang, stability::template::Template},
    db::parameters::ParameterData,
};
use sal_core::{dbg::Dbg, error::Error};
use std::collections::HashMap;

pub struct Summary {
    dbg: Dbg,
    title: String,
    table: Template,
}
//
impl Summary {
    pub fn from(
        parent: &Dbg,
        title: String, 
        language: &Lang,
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, ParameterData>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Summary");
        Ok(Self {
            dbg: dbg.clone(),
            title,
            table: Template::from_parameters(&dbg, language, target, result, ship_wide)?,
        })
    }
}
//
impl Content for Summary {
    //
    fn table(self) -> Result<String, Error> {
        self.table.to_string()
    }
    //
    fn title(&self) -> String {
        self.title.clone()        
    }
}
