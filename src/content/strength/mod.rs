use template::Template;

use crate::error::Error;

use super::Content;

pub mod table;
pub mod template;

//
pub struct Strength {
    shear_force: Template,
    bending_moment: Template,
}
//
impl Strength {
    pub fn new(shear_force: Template, bending_moment: Template) -> Self {
        Self {
            shear_force,
            bending_moment,
        }
    }
    //
    pub fn new_named(
        // x, sf, bm
        result: &[(f64, f64, f64)],
        // x, fr, sf, bm, limit_%
        target: &[(f64, i32, f64, f64, f64)],
        // fr, bm_min, bm_max, sf_min, sf_max
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
            .map(|(fr, bm_min, bm_max, sf_min, sf_max)| {
                (
                    (*fr, *sf_min * 0.001, *sf_max * 0.001),
                    (*fr, *bm_min * 0.001, *bm_max * 0.001),
                )
            })
            .unzip();
        Self::new(
            Template::new(
                "Перерезывающие силы".to_owned(),
                "SF".to_owned(),
                &sf_result,
                &sf_target,
                &sf_limit,
            ),
            Template::new(
                "Изгибающие моменты".to_owned(),
                "BM".to_owned(),
                &bm_result,
                &bm_target,
                &bm_limit,
            ),
        )
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok(self.shear_force.to_string()? + "\n" + &self.bending_moment.to_string()?)
    }
}
