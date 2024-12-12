pub struct Table {
    header: Vec<String>,
    // fr, min, doc, calc, max
    values: Vec<(i32, f64, f64, f64, f64)>,
}
//
impl Table {
    //
    pub fn new(name: &str, values: &[(i32, f64, f64, f64, f64)]) -> Self {
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
    pub fn new_header(
        header: &[&str],
        values: &[(i32, f64, f64, f64, f64)],
    ) -> Self {
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
        let limit = 5.;
        for (fr, min, target, result, max) in self.values {
            let delta_result_percent = (result - target).abs() * 100. / target;
            let state = match delta_result_percent <= limit {
                false => "-",
                true => "+",
            };
            string += &format!("|{fr}|{:3}|{:3}|{:3}|{:3}|{:2}| ±5 % | {state} |\n", min, target, result, max, delta_result_percent);
        }
        Ok(string + "  \n")
    }
}
