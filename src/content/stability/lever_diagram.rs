use crate::content::misc::{Curve, ICurve};

pub struct LeverDiagram {
    title: String,
    header: Vec<String>,
    // angle, dso
    target: Vec<(f64, f64, f64, f64)>,
    result: Vec<(f64, f64)>,
}
//
impl LeverDiagram {
    //
    pub fn new(
        title: &str,
        header: &[&str],
        target: &[(f64, f64, f64, f64)],
        result: &[(f64, f64)],
    ) -> Self {
        Self {
            title: title.to_owned(),
            header: header.iter().map(|s| s.to_string()).collect(),
            target: Vec::from(target),
            result: Vec::from(result),
        }
    }
    //
    pub fn from(language: &String, target: &[(f64, f64, f64, f64)], result: &[(f64, f64)]) -> Self {
        let (title, header) = if language.contains("en") {
            (
                "Stability curve",
                vec![
                    "№",
                    "Name",
                    "Dimension",
                    "Documentation",
                    "Calculation",
                    "%",
                    "Tolerances, %",
                    "Tolerances, abs",
                    "Status",
                ],
            )
        } else {
            (
                "Диаграмма статической остойчивости",
                vec![
                    "Крен",
                    "Плечо документация",
                    "Плечо расчет",
                    "%",
                    "Допуск, %",
                    "Допуск, абс.",
                    "Статус",
                ],
            )
        }
        .to_owned();
        Self::new(title, &header, target, result)
    }
    //
    pub fn to_string(self) -> Result<String, crate::error::Error> {
        let header = self
            .header
            .iter()
            .map(|s| format!("|{s}"))
            .collect::<String>()
            + "|\n"
            + &(0..self.header.len()).map(|_| "|---").collect::<String>()
            + "|\n";
        let mut string = format!("### {}\n\n", self.title) + &header;
        let result = Curve::new_linear(&self.result)?;
        for (angle, target, limit_p, limit_abs) in self.target {
            let result = result.value(angle as f64)?;
            let delta_result_abs = (result - target).abs();
            let delta_result_percent = if target != 0. {
                delta_result_abs * 100. / target.abs()
            } else {
                f64::MAX
            };
            let state = if delta_result_abs <= limit_abs || delta_result_percent <= limit_p {
                "+"
            } else {
                "-"
            };
            let delta_result_percent = if delta_result_percent != f64::MAX {
                format!("{:.2}", delta_result_percent)
            } else {
                format!(" ")
            };
            //    dbg!(&angle, &target, &result, delta_result_abs, delta_result_percent, limit_p, limit_abs, state);
            string += &format!(
                "|{}|{:.3}|{:.3}|{delta_result_percent}| ±{} % | ±{:.3} | {state} |\n",
                angle as i32, target, result, limit_p, limit_abs
            );
        }
        Ok(string)
    }
}
