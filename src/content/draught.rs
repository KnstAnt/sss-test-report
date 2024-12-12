use super::{misc::Table, Content};

pub struct Draught {
    table: Table,
}
//
impl Draught {
    pub fn new(values: &[(Option<f64>, Option<f64>)], ship_wide: f64) -> Self {
        Self {
            table: Table::new(
                &[&["3", "Осадка средняя", "м", "±1 %", "0,05 м"], 	
                            &["4", "Осадка на носовом перпендикуляре", "м", "±1 %", "0,05 м"],					
                            &["5", "Осадка на кормовом перпендикуляре", "м", "±1 %", "0,05 м"],	
                            &["6", "Дифферент", "град", "±1 %",],	
                            &["7", "Статический угол крена судна", "град", "±1 %",],					
                            &["51", "Дифферент", "м", ]], 
                values,
                ship_wide,
            ),            
        }
    }
}
//
impl Content for Draught {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        Ok("## Параметры посадки  \n".to_string() + &self.table.to_string()?)
    }
}
