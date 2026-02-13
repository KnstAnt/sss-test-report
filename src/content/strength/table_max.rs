use sal_core::{dbg::Dbg, error::Error};

use crate::content::misc::lang::Lang;

pub struct TableMax {
    dbg: Dbg,
    header: Vec<String>,
    // name, min, doc, calc, max, limit_%
    values: Vec<(String, f64, f64, f64, f64, f64)>,
}
//
impl TableMax {
    // parameter_name, min, doc, calc, max, limit_%
    pub fn new(
        parent: &Dbg,
        header: &[String],
        values: &[(String, f64, f64, f64, f64, f64)],
    ) -> Self {
        Self {
            dbg: Dbg::new(parent, "TableMax"),
            header: header.iter().map(|s| s.to_string()).collect(),
            values: Vec::from(values),
        }
    }
    //
    pub fn new_header(
        parent: &Dbg,
        language: &Lang,
        name: &str,
        values: &[(String, f64, f64, f64, f64, f64)],
    ) -> Self {
        let header = if *language == Lang::En {
            vec![
                "Parameter".to_string(),
                format!("${name}_{{min}}$"),
                "Documentation".to_string(),
                "Calculation".to_string(),
                format!("${name}_{{max}}$"),
                "%".to_string(),
                "Tolerances, %".to_string(),
                "Status".to_string(),
            ]
        } else {
            vec![
                "Параметр".to_string(),
                format!("${name}_{{min}}$"),
                "Документация".to_string(),
                "Расчет".to_string(),
                format!("${name}_{{max}}$"),
                "%".to_string(),
                "Допуск, %".to_string(),
                "Статус".to_string(),
            ]
        };
        Self::new(parent, &header, values)
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        let mut string = self
            .header
            .iter()
            .map(|s| format!("|{s}"))
            .collect::<String>()
            + "|\n"
            + &(0..self.header.len()).map(|_| "|---").collect::<String>()
            + "|\n";
        for (name, min, target, result, max, limit_p) in self.values {
            let delta = (result - target) * 100.;
            let delta_result_percent = if delta > 0. {
                if max != 0. {
                    delta / max
                } else {
                    0.
                }
            } else {
                if min != 0. {
                    delta / min
                } else {
                    0.
                }
            };
            let state = match delta_result_percent.abs() <= limit_p {
                false => "-",
                true => "+",
            };
            string += &format!(
                "|{name}|{:.3}|{:.3}|{:.3}|{:.3}|{:.2}| ±{} % | {state} |\n",
                min, target, result, max, delta_result_percent, limit_p as i32,
            );
        }
        Ok(string)
    }
}
