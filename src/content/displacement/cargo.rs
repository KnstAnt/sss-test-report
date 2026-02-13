use crate::{
    content::{Content, misc::lang::Lang}, db::cargo::CargoData
};
use sal_core::{dbg::Dbg, error::Error};
use super::table::Table;

pub struct Cargo {
    dbg: Dbg,
    title: String,
    table: Table,
}
//
impl Cargo {
    //
    pub fn new(dbg: Dbg, title: String, table: Table) -> Self {
        Self { dbg, title, table }
    }
    //
    pub fn from(parent: &Dbg, title: String, language: &Lang, data: &[CargoData]) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Cargo");
        let header = if *language == Lang::En { 
            vec!["Name", "Weight", "x_g [m]", "y_g [m]", "z_g [m]",]
        } else {
            vec!["Наименование", "Масса", "x_g [м]", "y_g [м]", "z_g [м]"]
        }.to_owned();
        let content = data
            .iter()
            .map(|v| {
                vec!(
                    v.name.clone().unwrap_or("-".to_string()), 
                    v.mass.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()),   
                    v.x_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
                    v.y_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
                    v.z_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
                )
            })
            .collect::<Vec<Vec<String>>>();
        Ok(Self::new(dbg.clone(), title, Table::new(&dbg, &header, content)))
    }
}
//
impl Content for Cargo {
    //
    fn table(self) -> Result<String, Error> {
        self.table.to_string()
    }
    //
    fn title(&self) -> String {
        self.title.clone()        
    }
}

