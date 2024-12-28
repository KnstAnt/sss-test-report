use template::Template;
use template_max::TemplateMax;

use crate::error::Error;

use super::Content;

pub mod table;
pub mod table_max;
pub mod template;
pub mod template_max;

//
pub struct Strength {
    shear_force: Template,
    shear_force_max: Option<TemplateMax>,
    bending_moment: Template,
    bending_moment_max: Option<TemplateMax>,
}
//
impl Strength {
    pub fn new(
        shear_force: Template,
        shear_force_max: Option<TemplateMax>,
        bending_moment: Template,
        bending_moment_max: Option<TemplateMax>,
    ) -> Self {
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
        // x, sf, bm, limit_%
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
            .map(|(x, fr, sf, bm, limit_p)| {
                (
                    (*x, *fr, *sf * 0.001, *limit_p),
                    (*x, *fr, *bm * 0.001, *limit_p),
                )
            })
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
                "BMmax_abs" => bm_max_abs = Some((row.1, row.2 * 0.001, row.3)),
                "BMmax_perc" => bm_max_percent = Some((row.1, row.2 * 0.001, row.3)),
                "SFmax_abs" => sf_max_abs = Some((row.1, row.2 * 0.001, row.3)),
                "SFmax_perc" => sf_max_percent = Some((row.1, row.2 * 0.001, row.3)),
                _ => panic!("Strength new_named error: wrong target_max!, row:{:?}", row),
            }
        }
        let (shear_force_max, bending_moment_max) = if let (
            Some(bm_max_abs),
            Some(bm_max_percent),
            Some(sf_max_abs),
            Some(sf_max_percent),
        ) = (bm_max_abs, bm_max_percent, sf_max_abs, sf_max_percent)
        {
            (
                Some(TemplateMax::new(
                    "SF".to_owned(),
                    &sf_result,
                    sf_max_abs,
                    sf_max_percent,
                    &sf_limit,
                )),
                Some(TemplateMax::new(
                    "BM".to_owned(),
                    &bm_result,
                    bm_max_abs,
                    bm_max_percent,
                    &bm_limit,
                )),
            )
        } else {
            (None, None)
        };
        Self::new(
            Template::new(
                "Перерезывающие силы".to_owned(),
                "SF".to_owned(),
                &sf_result,
                &sf_target,
                &sf_limit,
            ),
            shear_force_max,
            Template::new(
                "Изгибающие моменты".to_owned(),
                "BM".to_owned(),
                &bm_result,
                &bm_target,
                &bm_limit,
            ),
            bending_moment_max,
        )
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        let bending_moment_max = if let Some(bending_moment_max) = self.bending_moment_max {
            "\n".to_string() + &bending_moment_max.to_string()?
        } else {
            "".to_string()
        };
        let shear_force_max = if let Some(shear_force_max) = self.shear_force_max {
            "\n".to_string() + &shear_force_max.to_string()?
        } else {
            "".to_string()
        };    
        Ok("## Прочность\n\n".to_string()
            + &self
                .bending_moment
                .to_string()
                .map_err(|e| format!("Strength to_string bending_moment error:{}", e))?
            + &bending_moment_max
            + "\n"            
            + &self
                .shear_force
                .to_string()
                .map_err(|e| format!("Strength to_string shear_force error:{}", e))?
            + &shear_force_max)
    }
}
