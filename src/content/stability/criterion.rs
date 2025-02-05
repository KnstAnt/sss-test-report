use std::collections::HashMap;

use crate::{content::Content, db::criterion::CriteriaData, error::Error};

use super::template::Template;


pub struct Criterion {
    table: Template,
}
//
impl Criterion {
    pub fn from(
        language: &String,
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, CriteriaData>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        Ok(Self {
            table: Template::from_criterion(
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
    fn to_string(self) -> Result<String, crate::error::Error> {
        Ok("### Критерии\n\n".to_string() + &self.table.to_string()?)
    }
}
