pub struct TableMax {
    header: Vec<String>,
    // name, min, doc, calc, max, limit_%
    values: Vec<(String, f64, f64, f64, f64, f64)>,
}
//
impl TableMax {
    // parameter_name, min, doc, calc, max, limit_%
    pub fn new(name: &str, values: &[(String, f64, f64, f64, f64, f64)]) -> Self {
        Self::new_header(
            &vec![
                "Параметр",
                &format!("${name}_{{min}}$"),
                "Документация",
                "Расчет",
                &format!("${name}_{{max}}$"),
                "%",
                "Допуск, %",
                "Статус",
            ],
            values,
        )
    }
    //
    pub fn new_header(header: &[&str], values: &[(String, f64, f64, f64, f64, f64)]) -> Self {
        Self {
            header: header.iter().map(|s| s.to_string()).collect(),
            values: Vec::from(values),
        }
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
        for (name, min, target, result, max, limit_p) in self.values {
            let delta = result - target;            
            let delta_result_percent = if delta > 0. {
                delta * 100. / max
            } else {
                delta * 100. / min
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
