use std::collections::HashMap;
use criterion::Criterion;
use lever_diagram::LeverDiagram;
use parameters::Parameters;
use sal_core::{dbg::Dbg, error::Error};
use crate::{db::{criterion::CriteriaData, parameters::ParameterData}};
use super::Content;

pub mod unit;
pub mod lever_diagram;
pub mod template;
pub mod draught;
pub mod parameters;
pub mod criterion;

pub struct Stability {
    dbg: Dbg,
    title: String, 
    criterion: Criterion,
    lever_diagram: LeverDiagram,
    parameters: Parameters,
}
//
impl Stability {
    pub fn new(    
        dbg: Dbg,
        title: String, 
        criterion: Criterion,
        lever_diagram: LeverDiagram,
        parameters: Parameters,
    ) -> Self {
        Self {
            dbg,
            title, 
            criterion,
            lever_diagram,
            parameters,
        }
    }
    //
    pub fn new_named(
        parent: &Dbg,
        language: &String,
        criteria_target: &Vec<Vec<String>>,
        criteria_result: &HashMap<i32, CriteriaData>, // criterion_id, value        
        parameters_target: &Vec<Vec<String>>,
        parameters_result: &HashMap<i32, ParameterData>,
        ship_wide: f64,
        lever_diagram_target: &[(f64, f64, f64, f64)],
        lever_diagram_result: &[(f64, f64)],
    ) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Stability");
        let title = if language.contains("en") {
            "## Stability"
        } else {
            "## Остойчивость"
        }.to_owned();
        Ok(Self::new(
            dbg.clone(),
            title,
            Criterion::from(
                &dbg,
                language,
                criteria_target,
                criteria_result,
                ship_wide,
            )?,
            LeverDiagram::from(
                &dbg,
                language,
                lever_diagram_target,
                lever_diagram_result,
            ),
            Parameters::from(
                &dbg,
                language,
                parameters_target,
                parameters_result,
                ship_wide,
            )?,
        ))
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok(self.title + "\n\n" + 
            &self.criterion.to_string()? + "\n" + 
            &self.lever_diagram.to_string()? + "\n" + 
            &self.parameters.to_string()?
        )
    }
}
