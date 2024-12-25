use crate::error::Error;
use crate::content::Content;
use crate::content::misc::{Curve, ICurve};
//
pub struct Template {
    header: String,
    short_name: String,
    result: Vec<(f64, f64)>, //x, value
    target: Vec<(f64, i32, f64, f64)>, //x, fr, value, limit_%
    limit: Vec<(f64, f64, f64)>, //x, min, max
}
//
impl Template {
    //
    pub fn new( 
        header: String,
        short_name: String,
        result: &[(f64, f64)],
        target: &[(f64, i32, f64, f64)],
        limit: &[(f64, f64, f64)],
    ) -> Self {
        Self {
            header,
            short_name,
            result: Vec::from(result),
            target: Vec::from(target),
            limit: Vec::from(limit),
        }
    }
}
//
impl Content for Template {
    //
    fn to_string(self) -> Result<String, Error> {
        let (limit_min, limit_max): (Vec<(f64, f64)>, Vec<(f64, f64)>) = self.limit.into_iter().map(|(x, min, max)| ((x, min), (x, max))).unzip();
    //    let (fr_x, target) = self.target.into_iter().map(|(x, fr, v)| ((x, fr as f64), (fr, v))).unzip();
        let result = Curve::new_linear(&self.result).map_err(|e| format!("Template to_string result error:{}, src:{:?}", e, &self.result))?;
        let limit_min = Curve::new_linear(&limit_min).map_err(|e| format!("Template to_string limit_min error:{}, src:{:?}", e, &limit_min))?; 
        let limit_max = Curve::new_linear(&limit_max).map_err(|e| format!("Template to_string limit_max error:{}, src:{:?}", e, &limit_max))?; 
        let mut values = Vec::<(i32, f64, f64, f64, f64, f64)>::with_capacity(self.target.len());
        for (x, fr , target, limit_p) in self.target {
            // (fr, min, doc, calc, max, limit_%)
            values.push((fr, limit_min.value(x)?, target, result.value(x)?, limit_max.value(x)?, limit_p));
        };
        let content = format!("### {}\n", self.header) + &super::table::Table::new(&self.short_name, &values).to_string()?;
        Ok(content)
    } 
}
