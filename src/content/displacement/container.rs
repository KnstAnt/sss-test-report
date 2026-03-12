use super::table::Table;
use crate::{
    content::{misc::lang::Lang, Content},
    db::container::ContainerData,
};
use sal_core::error::Error;

pub struct Container {
    title: String,
    table: Table,
}
//
impl Container {
    //
    pub fn new(title: String, table: Table) -> Self {
        Self { title, table }
    }
    //
    pub fn from(
        title: String,
        language: &Lang,
        data: &[ContainerData],
    ) -> Self {
        let header = if *language == Lang::En {
            vec!["Name", "BBRRTT", "Weight", "x_g [m]", "y_g [m]", "z_g [m]"]
        } else {
            vec![
                "Наименование",
                "BBRRTT",
                "Масса",
                "x_g [м]",
                "y_g [м]",
                "z_g [м]",
            ]
        }
        .to_owned();
        let content = data
            .iter()
            .map(|v| {
                vec![
                    v.name(),
                    v.bbrrtt(),
                    format!("{:.3}", v.mass),
                    format!("{:.3}", v.x_g),
                    format!("{:.3}", v.y_g),
                    format!("{:.3}", v.z_g),
                ]
            })
            .collect::<Vec<Vec<String>>>();
        Self::new(title, Table::new(&header, content))
    }
}
//
impl Content for Container {
    //
    fn to_string(self) -> Result<String, Error> {
        Ok(format!("{}\n\n{}", self.title, self.table.to_string()))
    }
}
