use std::collections::HashMap;
use sal_core::{dbg::Dbg, error::Error};
use crate::{content::{Content, misc::lang::Lang}, db::criterion::CriteriaData};
use super::template::Template;


pub struct Criterion {
    dbg: Dbg,
    title: String,
    table: Template,
}
//
impl Criterion {
    pub fn from(
        parent: &Dbg,
        language: &Lang,
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, CriteriaData>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Criterion");
        let title = if *language == Lang::En {
            "Criteria"
        } else  {
            "Критерии"
        }.to_owned();
        Ok(Self {
            dbg: dbg.clone(),
            title,
            table: Template::from_criterion(
                &dbg,
                language,
                &target.clone().into_iter().skip(1).collect::<Vec<_>>(),
                result,
                ship_wide,
            )?,            
        })
    }
}
//
impl Content for Criterion {
    //
    fn to_string(self) -> Result<String, Error> {
        Ok(format!("### {}\n\n", self.title) + &self.table.to_string()?)
    }
}
