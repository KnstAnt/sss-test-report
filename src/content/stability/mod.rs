use std::collections::HashMap;

use criterion::Criterion;
use lever_diagram::LeverDiagram;
use parameters::Parameters;

use crate::{db::{criterion::CriteriaData, parameters::ParameterData}, error::Error};

use super::Content;

pub mod unit;
pub mod lever_diagram;
pub mod template;
pub mod draught;
pub mod parameters;
pub mod criterion;

pub struct Stability {
    title: String, 
    criterion: Criterion,
    lever_diagram: LeverDiagram,
    parameters: Parameters,
}
//
impl Stability {
    pub fn new(    
        title: String, 
        criterion: Criterion,
        lever_diagram: LeverDiagram,
        parameters: Parameters,
    ) -> Self {
        Self {
            title, 
            criterion,
            lever_diagram,
            parameters,
        }
    }
    //
    pub fn new_named(
        language: &String,
        criteria_target: &Vec<Vec<String>>,
        criteria_result: &HashMap<i32, CriteriaData>, // criterion_id, value        
        parameters_target: &Vec<Vec<String>>,
        parameters_result: &HashMap<i32, ParameterData>,
        ship_wide: f64,
        lever_diagram_target: &[(f64, f64, f64, f64)],
        lever_diagram_result: &[(f64, f64)],
    ) -> Result<Self, Error> {
        let title = if language.contains("en") {
            "## Stability"
        } else {
            "## Остойчивость"
        }.to_owned();
        Ok(Self::new(
            title,
            Criterion::from(
                language,
                criteria_target,
                criteria_result,
                ship_wide,
            )?,
            LeverDiagram::from(
                language,
                lever_diagram_target,
                lever_diagram_result,
            ),
            Parameters::from(
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
