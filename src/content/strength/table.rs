pub struct Table {
    header: Vec<String>,
    // fr, min, doc, calc, max, limit_%
    values: Vec<(i32, f64, f64, f64, f64, f64)>,
}
//
impl Table {
    // fr, min, doc, calc, max, limit_%
    pub fn new(name: &str, values: &[(i32, f64, f64, f64, f64, f64)]) -> Self {
        Self::new_header(
            &vec![
                "Fr",
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
    pub fn new_header(header: &[&str], values: &[(i32, f64, f64, f64, f64, f64)]) -> Self {
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
        for (fr, min, target, result, max, limit) in self.values {
            let delta = result - target;            
            let delta_result_percent = if delta > 0. {
                delta * 100. / max
            } else {
                delta * 100. / min
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
