use crate::{
    content::{Content, misc::lang::Lang}, db::bulk_cargo::BulkCargoData
};
use super::table::Table;
use sal_core::error::Error;

pub struct BulkCargo {
    title: String,
    table: Table,
}
//
impl BulkCargo {
    //
    pub fn new( title: String, table: Table) -> Self {
        Self {title, table }
    }
    //
    pub fn from(title: String, language: &Lang, data: &[BulkCargoData]) -> Self {
        let header = if *language == Lang::En { 
            vec!["Name", "Weight", "x_g [m]", "y_g [m]", "z_g [m]", "Grain moment [tm]"]
        } else {
            vec!["Наименование", "Масса", "x_g [м]", "y_g [м]", "z_g [м]", "Кренящий момент от смещения зерна [тм]"]
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
                    v.grain_moment.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()),
                )
            })
            .collect::<Vec<Vec<String>>>();
        Self::new(title, Table::new(&header, content))
    }
}
//
impl Content for BulkCargo {
    //
    fn to_string(self) -> Result<String, Error> {
        Ok(format!("{}\n\n{}", self.title, self.table.to_string()))
    }
}
