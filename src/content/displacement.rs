use super::{table::Table, Content};

pub struct Displacement {
    table: Table,
}
//
impl Displacement {
    pub fn new(values: &[(Option<f64>, Option<f64>)]) -> Self {
        Self {
            table: Table::new(
                &[&["2", "Водоизмещение весовое", "т", "±2 %"], 	
                            &["32", "Абсцисса центра тяжести", "м"],					
                            &["56", "Абсцисса центра тяжести от кормового перпендикуляра", "м", "±1 %", "0,50 м"],	
                            &["12", "Аппликата центра тяжести", "м", "±1 %", "0,05 м"],	
                            &["1", "Аппликата центра тяжести исправленная", "м"],					
                            &["52", "Ордината центра тяжести", "м", "±0,5 % ширины судна", "0,05 м"]], 
                values
            ),
        }
    }
}
//
impl Content for Displacement {
    //
    fn to_string(self) -> String {
        "# Водоизмещение  \n".to_string() + &self.table.to_string()
    }
}
