use std::collections::HashMap;

use criterion::Criterion;
use lever_diagram::LeverDiagram;
use parameters::Parameters;

use crate::error::Error;

use super::Content;

pub mod unit;
pub mod lever_diagram;
pub mod template;
pub mod displacement;
pub mod draught;
pub mod parameters;
pub mod criterion;

pub struct Stability {
    criterion: Criterion,
    lever_diagram: LeverDiagram,
    parameters: Parameters,
}
//
impl Stability {
    pub fn new(    
        criterion: Criterion,
        lever_diagram: LeverDiagram,
        parameters: Parameters,
    ) -> Self {
        Self {
            criterion,
            lever_diagram,
            parameters,
        }
    }
    //
    pub fn new_named(
        criteria_target: &Vec<Vec<String>>,
        criteria_result: &HashMap<i32, f64>, // criterion_id, value        
        parameters_target: &Vec<Vec<String>>,
        parameters_result: &HashMap<i32, f64>,
        ship_wide: f64,
        lever_diagram_target: &[(f64, f64, f64, f64)],
        lever_diagram_result: &[(f64, f64)],
    ) -> Result<Self, Error> {
        Ok(Self::new(
            Criterion::from_data(
                criteria_target,
                criteria_result,
                ship_wide,
            )?,
            LeverDiagram::new(
                lever_diagram_target,
                lever_diagram_result,
            ),
            Parameters::from_data(
                parameters_target,
                parameters_result,
                ship_wide,
            )?,
        ))
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok("## Остойчивость".to_string() + "\n" + 
            &self.criterion.to_string()? + "\n" + 
            &self.lever_diagram.to_string()? + "\n" + 
            &self.parameters.to_string()?
        )
    }
}
