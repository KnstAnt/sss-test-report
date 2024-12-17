use std::collections::HashMap;
use crate::{content::Content, error::Error};
use super::unit::TableUnit;

//
pub struct Template {
    header: Vec<String>,
    data: Vec<TableUnit>,
    ship_wide: f64,
}
//
impl Template {
    pub fn new(header:&[String], data: &[TableUnit], ship_wide: f64,) -> Self {
        Self {
            header: Vec::from(header),
            data: Vec::from(data),
            ship_wide,
        }
    }
    //
    pub fn from_string(
        target: &Vec<Vec<String>>,
        result: &HashMap<i32, f64>,
        ship_wide: f64,
    ) -> Result<Self, Error> {
        let mut data = Vec::new();
        for row in target.iter() {
            data.push(TableUnit::from_data(row, result)?);
        }
        Ok(Self::new(
            &vec![
                "№".to_string(),
                "Наименование".to_string(),
                "Размерность".to_string(),
                "Документация".to_string(),
                "Расчет".to_string(),
                "%".to_string(),
                "Допуск, %".to_string(),
                "Допуск, абс.".to_string(),
                "Статус".to_string(),
            ],
            &data,
            ship_wide,
        ))
    }
}
//
impl Content for Template {
    //
    fn to_string(self) -> Result<String, Error> {
        let mut string = self
            .header
            .iter()
            .map(|s| format!("|{s}"))
            .collect::<String>()
            + "|\n"
            + &(0..self.header.len()).map(|_| "|---").collect::<String>()
            + "|\n";
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
        let print_abs = |v: Option<f64>| v.map_or("".to_string(), |v| format!("{:.3}", v));
        let print_percent = |v: Option<f64>| v.map_or("".to_string(), |v| format!("{:.2}", v));
        let print_str = |v: &Option<String>| v.clone().map_or("".to_owned(), |v| v.to_string());
        for data in self.data {
        //    dbg!(&data);
            let (target, result) = (data.target, data.result);
            let (delta_result_abs, delta_result_percent) = match (target, result) {
                (Some(target), Some(result)) => {
                    let delta = (result - target).abs();
                //    dbg!(&result, &target, &delta);
                    (Some(delta), Some(delta * 100. / target))
                }
                _ => (None, None),
            };
            let process_limit = |limit: &Option<String>| -> (Option<bool>, String) {
        //        dbg!(&delta_result_percent, &delta_result_abs, &limit);
                let limit_res = if let Some(limit) = limit {
                    if limit.contains('%') {
                        if limit.contains("ширины судна") {
                            if let (Some(delta), Some(limit)) = (delta_result_abs, parse_limit(limit)) {  
                                Some(delta <= self.ship_wide*limit/100.)
                            } else {
                                None
                            }
                        } else {
                            check_limit(delta_result_percent, limit)
                        }
                    } else {
                        check_limit(delta_result_abs, limit)
                    }
                } else {
                    None
                };
                let limit_str = print_str(&limit);
                (limit_res, limit_str.to_owned())
            };
        //    dbg!(&data.limit_percent);
            let target = print_abs(target);
            let result = print_abs(result);
            let id = data.id;
            let name = data.name;
            let unit = data.unit;
            let (limit_res_p, limit_str_p) = process_limit(&data.limit_percent);
            let (limit_res_abs, limit_str_abs) = process_limit(&data.limit_abs);
           // dbg!(&data.limit_abs, &limit_res_abs, &limit_str_abs);
            let state = match (limit_res_p, limit_res_abs) {
                (Some(false), _) | (_, Some(false)) => "-",
                (None, None) => "",
                _ => "+",
            };
            let delta_result_percent = print_percent(delta_result_percent);
           // dbg!(&target, &limit_str_abs);
            string += &format!("|{id}|{name}|{unit}|{target}|{result}|{delta_result_percent}|{limit_str_p}|{limit_str_abs}|{state}|\n");
        }
        Ok(string)
    }
}
