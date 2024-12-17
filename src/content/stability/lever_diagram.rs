use crate::content::misc::{Curve, ICurve};

pub struct LeverDiagram {
    // angle, dso
    target: Vec<(f64, f64, f64, f64)>,
    result: Vec<(f64, f64)>,
}
//
impl LeverDiagram {
    //
    pub fn new(target: &[(f64, f64, f64, f64)], result: &[(f64, f64)]) -> Self {
        Self {
            target: Vec::from(target),
            result: Vec::from(result),
        }
    }
    //
    pub fn to_string(self) -> Result<String, crate::error::Error> {
        let mut string = "### Диаграмма статической остойчивости\n".to_owned() + 
        &"| Крен | Плечо документация | Плечо расчет | %   | Допуск % | Допуск, абс. | Статус |\n"  +
        &"|---|---|---|---|---|---|---|\n";
        let result = Curve::new_linear(&self.result)?;
        for (angle, target, limit_p, limit_abs) in self.target {
            let result = result.value(angle as f64)?;
            let delta_result_abs = (result - target).abs();
            let delta_result_percent = (delta_result_abs * 100. / target).abs();
            let state = if delta_result_percent <= limit_abs && delta_result_percent <= limit_p {
                "+"
            } else {
                "-"
            };
            string += &format!(
                "|{}|{:.3}|{:.3}|{:.2}| ±{:.2} % | ±{:.3} см | {state} |\n",
                angle as i32, target, result, delta_result_percent, limit_p, limit_abs
            );
        }
        Ok(string)
    }
}
