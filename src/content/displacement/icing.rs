use super::table::Table;
use crate::{
    content::{misc::lang::Lang, Content},
    db::parameters::ParameterData,
};
use sal_core::{dbg::Dbg, error::Error};
use std::collections::HashMap;

pub struct Icing {
    title: String,
    table: Table,
}
//
impl Icing {
    //
    pub fn new(title: String, table: Table) -> Self {
        Self { title, table }
    }
    //
    pub fn from(
        parent: &Dbg,
        title: String,
        language: &Lang,
        parameters: &HashMap<i32, ParameterData>,
    ) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Icing");
        let error = Error::new(&dbg, "from");
        let header = if *language == Lang::En {
            vec!["Name", "Weight [t]", "x_g [m]", "y_g [m]", "z_g [m]"]
        } else {
            vec!["Наименование", "Масса [т]", "x_g [м]", "y_g [м]", "z_g [м]"]
        }
        .to_owned();
        let (ice_h_name, ice_v_name) = if *language == Lang::En {
            (
                "Ice on horizontal surfaces".to_owned(),
                "Ice on vertical surfaces".to_owned(),
            )
        } else {
            (
                "Обледенение горизонтальных поверхностей".to_owned(),
                "Обледенение вертикальных поверхностей".to_owned(),
            )
        };
        let print_value = |id| -> Result<String, Error> {
            Ok(parameters
                .get(&id)
                .ok_or(error.err(format!("parameters.get({})", id)))?
                .result
                .map(|v| format!("{:.3}", v))
                .unwrap_or(" ".to_owned()))
        };
        let ice_h = vec![
            ice_h_name,
            print_value(109)?,
            print_value(110)?,
            print_value(111)?,
            print_value(112)?,
        ];
        let ice_v = vec![
            ice_v_name,
            print_value(113)?,
            print_value(114)?,
            print_value(115)?,
            print_value(116)?,
        ];
        let content = vec![ice_h, ice_v];
        Ok(Self::new(
            title,
            Table::new(&header, content),
        ))
    }
}
//
impl Content for Icing {
    //
    fn to_string(self) -> Result<String, Error> {
        Ok(format!("{}\n\n{}", self.title, self.table.to_string()))
    }
}
