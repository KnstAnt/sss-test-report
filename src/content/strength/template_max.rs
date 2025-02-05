use crate::error::Error;
use crate::content::Content;
use crate::content::misc::{Curve, ICurve};
//
pub struct TemplateMax {
    name: String,
    language: String,
    result: Vec<(f64, f64)>, //x, value
    target_abs: (f64, f64, f64), //x, value, limit_%
    target_percent: (f64, f64, f64), //x, value, limit_%
    limit: Vec<(f64, f64, f64)>, //x, min, max
}
//
impl TemplateMax {
    //
    pub fn new( 
        name: String,
        language: &String,
        result: &[(f64, f64)],
        target_abs: (f64, f64, f64),
        target_percent: (f64, f64, f64),
        limit: &[(f64, f64, f64)],
    ) -> Self {
        Self {
            name,
            language: language.clone(),
            result: Vec::from(result),
            target_abs,
            target_percent,
            limit: Vec::from(limit),
        }
    }
}
//
impl Content for TemplateMax {
    //
    fn to_string(self) -> Result<String, Error> {
        let (limit_min, limit_max): (Vec<(f64, f64)>, Vec<(f64, f64)>) = self.limit.into_iter().map(|(x, min, max)| ((x, min), (x, max))).unzip();
    //    let (fr_x, target) = self.target.into_iter().map(|(x, fr, v)| ((x, fr as f64), (fr, v))).unzip();
        let result = Curve::new_linear(&self.result)?;
        let limit_min = Curve::new_linear(&limit_min)?; 
        let limit_max = Curve::new_linear(&limit_max)?; 
        let mut values = Vec::new();
        let x = self.target_abs.0;
        let (value_str, percent_str) = if self.language.contains("en") {
            ("Maximum value", "Maximum percent")
        } else {
            ("Максимальное значение", "Максимальный процент")
        };
        values.push((value_str.to_owned(), limit_min.value(x)?, self.target_abs.1, result.value(x)?, limit_max.value(x)?, self.target_abs.2));
        let x = self.target_percent.0;
        values.push((percent_str.to_owned(), limit_min.value(x)?, self.target_percent.1, result.value(x)?, limit_max.value(x)?, self.target_percent.2));
        super::table_max::TableMax::new_header(&self.language, &self.name, &values).to_string()
    } 
}
