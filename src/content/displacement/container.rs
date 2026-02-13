use crate::{
    content::{Content, misc::lang::Lang}, db::container::ContainerData
};
use sal_core::{dbg::Dbg, error::Error};
use super::table::Table;

pub struct Container {
    dbg: Dbg,
    title: String,
    table: Table,
}
//
impl Container {
    //
    pub fn new(dbg: Dbg, title: String, table: Table) -> Self {
        Self { dbg, title, table }
    }
    //
    pub fn from(parent: &Dbg, title: String, language: &Lang, data: &[ContainerData]) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Container");
        let header = if *language == Lang::En { 
            vec!["Name", "BBRRTT", "Weight", "x_g [m]", "y_g [m]", "z_g [m]",]
        } else {
            vec!["Наименование", "BBRRTT", "Масса", "x_g [м]", "y_g [м]", "z_g [м]"]
        }.to_owned();
        let content = data
            .iter()
            .map(|v| {
                vec!(
                    v.name(), 
                    v.bbrrtt(),
                    format!("{:.3}", v.mass), 
                    format!("{:.3}", v.x_g), 
                    format!("{:.3}", v.y_g),  
                    format!("{:.3}", v.z_g), 
                )
            })
            .collect::<Vec<Vec<String>>>();
        Ok(Self::new(dbg.clone(), title, Table::new(&dbg, &header, content)))
    }
}
//
impl Content for Container {
    //
    fn to_string(self) -> Result<String, Error> {
        Ok(format!("{}\n\n{}", self.title, self.table.to_string()))
    }
}

