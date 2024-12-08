use super::Content;

pub struct Table {
    header: Vec<String>,
    content: Vec<Vec<String>>,
    values: Vec<(Option<f64>, Option<f64>)>,
    ship_wide: f64,
}
//
impl Table {
    //
    pub fn new(content: &[&[&str]], values: &[(Option<f64>, Option<f64>)], ship_wide: f64) -> Self {
        Self::new_header(
            &vec![
                "№",
                "Наименование",
                "Размерность",
                "Документация",
                "Расчет",
                "%",
                "Допуск, %",
                "Допуск, абс.",
                "Статус",
            ],
            content,
            values,
            ship_wide,
        )
    }
    //
    pub fn new_header(
        header: &[&str],
        content: &[&[&str]],
        values: &[(Option<f64>, Option<f64>)],
        ship_wide: f64,
    ) -> Self {
        Self {
            header: header.iter().map(|s| s.to_string()).collect(),
            content: content
                .iter()
                .map(|v| v.iter().map(|s| s.to_string()).collect())
                .collect(),
            values: Vec::from(values),
            ship_wide,
        }
    }
}
//
impl Content for Table {
    //
    fn to_string(self) -> String {
        let mut string = self
            .header
            .iter()
            .map(|s| format!("|{s}"))
            .collect::<String>()
            + "|\n"
            + &(0..self.header.len()).map(|_| "|---").collect::<String>()
            + "|\n";
        let mut values = self.values.iter();
        let parse_limit = |s: &String| -> Option<f64> {
            s.replace(',', ".")
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '.')
                .collect::<String>()
                .parse::<f64>()
                .ok()
        };
        let check_limit = |delta: Option<f64>, limit: &String| -> Option<bool> {
            if let (Some(delta), Some(limit)) = (delta, parse_limit(limit)) {
                Some(delta <= limit)
            } else {
                None
            }
        };
        let print_value = |v: Option<f64>| v.map_or("".to_string(), |v| v.to_string());
        let print_str = |v: &Option<&String>| v.map_or("".to_owned(), |v| v.to_string());
        for content in self.content {
            let &(target, result) = values.next().expect("Content table error: no values!");
            let (delta_result_abs, delta_result_percent) = match (target, result) {
                (Some(target), Some(result)) => {
                    let delta = (result - target).abs();
                    (Some(delta), Some(delta * 100. / target))
                }
                _ => (None, None),
            };
            let process_limit = |limit: &Option<&String>| -> (Option<bool>, String){
                let limit_res = if let Some(limit) = limit {
                    if limit.contains('%') {
                        check_limit(delta_result_percent, limit)
                    } else {
                        check_limit(delta_result_abs, limit)
                    }
                } else {
                    None
                };
                let limit_str = print_str(limit);
                (limit_res, limit_str)
            };
            let target = print_value(target);
            let result = print_value(result);
            let n = &content[0];
            let name = &content[1];
            let unit = print_str(&content.get(2));
            let (limit_res1, limit_str1) = process_limit(&content.get(3));
            let (limit_res2, limit_str2) = process_limit(&content.last());
            let state = match (limit_res1, limit_res2) {
                (Some(false), _) | (_, Some(false)) => "-",
                (None, None) => "",
                _ => "+",
            };
            let delta_result_percent = print_value(delta_result_percent);
            string += &format!("|{n}|{name}|{unit}|{target}|{result}|{delta_result_percent}|{limit_str1}|{limit_str2}|{state}|");
        }
        string + "  \n"
    }
}
