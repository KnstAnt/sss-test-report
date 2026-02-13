use super::table::Table;
use crate::{content::Content, db::tank::TankData};
use sal_core::{dbg::Dbg, error::Error};

pub struct Tank {
    dbg: Dbg,
    table: Table,
}
//
impl Tank {
    //
    pub fn new(dbg: Dbg, table: Table) -> Self {
        Self { dbg, table }
    }
    //
    pub fn from(parent: &Dbg, language: &String, data: &[TankData]) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Tank");
        let header = if language.contains("en") {
            vec![
                "Name",
                "Weight",
                "x_g [m]",
                "y_g [m]",
                "z_g [m]",
                "M_f.sx [tm]",
            ]
        } else {
            vec![
                "Наименование",
                "Масса",
                "x_g [м]",
                "y_g [м]",
                "z_g [м]",
                "M_f.sx [тм]",
            ]
        }
        .to_owned();
        let content = data
            .iter()
            .map(|v| {
                vec![
                    v.name.clone().unwrap_or("-".to_string()),
                    v.mass
                        .map(|v| format!("{:.3}", v))
                        .unwrap_or("-".to_string()),
                    v.x_g
                        .map(|v| format!("{:.3}", v))
                        .unwrap_or("-".to_string()),
                    v.y_g
                        .map(|v| format!("{:.3}", v))
                        .unwrap_or("-".to_string()),
                    v.z_g
                        .map(|v| format!("{:.3}", v))
                        .unwrap_or("-".to_string()),
                    v.f_sx
                        .map(|v| format!("{:.3}", v))
                        .unwrap_or("-".to_string()),
                ]
            })
            .collect::<Vec<Vec<String>>>();
        Ok(Self::new(dbg.clone(), Table::new(&dbg, &header, content)))
    }
}
//
impl Content for Tank {
    //
    fn to_string(self) -> Result<String, Error> {
        self.table.to_string()
    }
}
