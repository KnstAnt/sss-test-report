pub struct Table {
    header: Vec<String>,
    // fr, min, doc, calc, max, limit_%
    values: Vec<(i32, f64, f64, f64, f64, f64)>,
}
//
impl Table {
    // fr, min, doc, calc, max, limit_%
    pub fn new(header: &[String], values: &[(i32, f64, f64, f64, f64, f64)]) -> Self {
        Self {
            header: header.iter().map(|s| s.to_string()).collect(),
            values: Vec::from(values),
        }
    }
    //
    pub fn from(
        language: &String,
        name: &str,
        values: &[(i32, f64, f64, f64, f64, f64)],
    ) -> Self {
        let header = if language.contains("en") {
            vec![
                "Fr".to_string(),
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
                "Fr".to_string(),
                format!("${name}_{{min}}$"),
                "Документация".to_string(),
                "Расчет".to_string(),
                format!("${name}_{{max}}$"),
                "%".to_string(),
                "Допуск, %".to_string(),
                "Статус".to_string(),
            ]
        };
        Self::new(&header, values)
    }
    //
    pub fn to_string(self) -> Result<String, crate::error::Error> {
        let mut string = self
            .header
            .iter()
            .map(|s| format!("|{s}"))
            .collect::<String>()
            + "|\n"
            + &(0..self.header.len()).map(|_| "|---").collect::<String>()
            + "|\n";
        for (fr, min, target, result, max, limit) in self.values {
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
            let state = match delta_result_percent.abs() <= limit {
                false => "-",
                true => "+",
            };
            //   dbg!(result, target, delta, delta_result_percent);
            string += &format!(
                "|{fr}|{:.3}|{:.3}|{:.3}|{:.3}|{:.2}| ±{} % | {state} |\n",
                min, target, result, max, delta_result_percent, limit as i32,
            );
        }
        Ok(string)
    }
}
