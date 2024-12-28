use super::unit::TableUnit;
use crate::{content::Content, error::Error};
use std::collections::HashMap;

//
pub struct Template {
    header: Vec<String>,
    data: Vec<TableUnit>,
    ship_wide: f64,
}
//
impl Template {
    pub fn new(header: &[String], data: &[TableUnit], ship_wide: f64) -> Self {
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
        let print_abs = |v: Option<f64>| v.map_or(" ".to_string(), |v| format!("{:.3}", v));
        let print_percent = |v: Option<f64>| v.map_or(" ".to_string(), |v| format!("{:.2}", v));
        let print_str = |v: &Option<String>| v.clone().map_or(" ".to_owned(), |v| v.replace("-", &" ").to_string());
        for data in self.data {
            let (target, result) = (data.target, data.result);
            let (delta_result_abs, mut delta_result_percent) = match (target, result) {
                (Some(target), Some(result)) => {
                    let delta_abs = (result - target).abs();
                    let delta_percent = if target != 0. {
                        Some(delta_abs * 100. / target)
                    } else {
                        None
                    };
                    (Some(delta_abs), delta_percent)
                }
                _ => (None, None),
            };
            let mut process_limit_percent = |limit: &Option<String>| -> (Option<bool>, String) {
                let limit_res = if let Some(limit) = limit {
                    if limit.contains("ширины судна") {
                        if let (Some(delta), Some(limit)) = (delta_result_abs, parse_limit(limit)) {
                            if self.ship_wide > 0. {
                                delta_result_percent = Some(delta * 100. / self.ship_wide);
                                Some(delta <= self.ship_wide * limit / 100.)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        check_limit(delta_result_percent, limit)
                    }
                } else {
                    None
                };
                let limit_str = print_str(&limit);
                (limit_res, limit_str.to_owned())
            };
            let process_limit_abs = |limit: &Option<String>| -> (Option<bool>, String) {
                let limit_res = if let Some(limit) = limit {
                    check_limit(delta_result_abs, limit)
                } else {
                    None
                };
                let limit_str = print_str(&limit);
                (limit_res, limit_str.to_owned())
            };
            let target = print_abs(target);
            let result = print_abs(result);
            let id = data.id;
            let name = data.name;
            let unit = data.unit.replace("-", &" ");
            let (limit_res_p, limit_str_p) = process_limit_percent(&data.limit_percent);
            let (limit_res_abs, limit_str_abs) = process_limit_abs(&data.limit_abs);
            let state = match (limit_res_p, limit_res_abs) {
                (Some(true), _) | (_, Some(true)) => "+",
                (None, None) => " ",
                _ => "-",
            };
            let delta_result_percent = print_percent(delta_result_percent);
            string += &format!("|{id}|{name}|{unit}|{target}|{result}|{delta_result_percent}|{limit_str_p}|{limit_str_abs}|{state}|\n");
        }
        Ok(string)
    }
}
