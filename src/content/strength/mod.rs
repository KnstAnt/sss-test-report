use template_max::TemplateMax;
use template::Template;

use crate::error::Error;

use super::Content;

pub mod table;
pub mod template;
pub mod table_max;
pub mod template_max;

//
pub struct Strength {
    shear_force: Template,
    shear_force_max: TemplateMax,
    bending_moment: Template,
    bending_moment_max: TemplateMax,
}
//
impl Strength {
    pub fn new(shear_force: Template, shear_force_max: TemplateMax, bending_moment: Template, bending_moment_max: TemplateMax,) -> Self {
        Self {
            shear_force,
            shear_force_max,
            bending_moment,
            bending_moment_max,
        }
    }
    //
    pub fn new_named(
        // x, sf, bm
        result: &[(f64, f64, f64)],
        // x, fr, sf, bm, limit_%
        target: &[(f64, i32, f64, f64, f64)],
        // x, fr, sf, bm, limit_%
        target_max: &[(String, f64, f64, f64)],
        // (frame_x, bm_min, bm_max, sf_min, sf_max)
        limit: &[(f64, f64, f64, f64, f64)],
    ) -> Self {
        let (sf_result, bm_result): (Vec<_>, Vec<_>) = result
            .iter()
            .map(|(x, sf, bm)| ((*x, *sf * 0.001), (*x, *bm * 0.001)))
            .unzip();
        let (sf_target, bm_target): (Vec<_>, Vec<_>) = target
            .iter()
            .map(|(x, fr, sf, bm, limit_p)| ((*x, *fr, *sf * 0.001, *limit_p), (*x, *fr, *bm * 0.001, *limit_p)))
            .unzip();
        let (sf_limit, bm_limit): (Vec<_>, Vec<_>) = limit
            .iter()
            .map(|(x, bm_min, bm_max, sf_min, sf_max)| {
                (
                    (*x, *sf_min * 0.001, *sf_max * 0.001),
                    (*x, *bm_min * 0.001, *bm_max * 0.001),
                )
            })
            .unzip();
        let mut sf_max_abs = None;
        let mut sf_max_percent = None;
        let mut bm_max_abs = None;
        let mut bm_max_percent = None;      
        for row in target_max {
            match row.0.as_str() {
                "BMmax_abs" => sf_max_abs = Some((row.1, row.2, row.3)),
                "BMmax_perc" => sf_max_percent = Some((row.1, row.2, row.3)),
                "SFmax_abs" => bm_max_abs = Some((row.1, row.2, row.3)),
                "SFmax_perc" => bm_max_percent = Some((row.1, row.2, row.3)),
                _ => panic!("Strength new_named error: wrong target_max!, row:{:?}", row),
            }
        }
        Self::new(
            Template::new(
                "Перерезывающие силы".to_owned(),
                "SF".to_owned(),
                &sf_result,
                &sf_target,
                &sf_limit,
            ),
            TemplateMax::new(
                "SF".to_owned(),
                &sf_result,
                sf_max_abs.unwrap(),
                sf_max_percent.unwrap(),
                &sf_limit,
            ),
            Template::new(
                "Изгибающие моменты".to_owned(),
                "BM".to_owned(),
                &bm_result,
                &bm_target,
                &bm_limit,
            ),
            TemplateMax::new(
                "BM".to_owned(),
                &bm_result,
                bm_max_abs.unwrap(),
                bm_max_percent.unwrap(),
                &bm_limit,
            ),
        )
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok("## Прочность".to_string() + "\n" + 
            &self.bending_moment.to_string()? + "\n" + 
            &self.bending_moment_max.to_string()? + "\n" + 
            &self.shear_force.to_string()? + "\n" + 
            &self.shear_force_max.to_string()?
        )
    }
}
