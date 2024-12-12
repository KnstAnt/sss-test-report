//! Промежуточные структуры для serde_json для парсинга данных
//! расчета прочности 
use serde::{Deserialize, Serialize};

use super::DataArray;
/// Данные расчета прочности 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrengthLimitData {
    pub fr: f64,
    pub value: f64,
    pub limit_type: String, //'low', 'high'
    pub limit_area: String, //'sea', 'harbor'
    pub force_type: String, //'shear_force', 'bending_moment'
}
//
impl std::fmt::Display for StrengthLimitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StrengthLimitData(sf:{}, value:{}, limit_type:{}, limit_area:{}, force_type:{} )",
            self.fr, self.value, self.limit_type, self.limit_area, self.force_type,
        )
    }
}
pub type StrengthLimitDataArray = DataArray<StrengthLimitData>;
//
impl StrengthLimitDataArray {
    // (fr, bm_min, bm_max, sf_min, sf_max)
    pub fn data(&self, area: &str) -> Vec<(f64, f64, f64, f64, f64)> {
        convert(&self
            .data
            .iter()
            .filter(|v| v.limit_area.contains(area))
            .collect::<Vec<_>>()
        )        
    }
}
//
fn convert(data: &Vec<&StrengthLimitData>) -> Vec<(f64, f64, f64, f64, f64)> {
    let mut sf_min = Vec::new();
    let mut sf_max = Vec::new();
    let mut bm_min = Vec::new();
    let mut bm_max = Vec::new();
    for v in data {
        if v.force_type.contains("shear_force") {
            if v.limit_type.contains("low") {
                sf_min.push((v.fr, v.value));
            } else {// 'high'
                sf_max.push((v.fr, v.value));
            }
        } else { // 'bending_moment'
            if v.limit_type.contains("low") {
                bm_min.push((v.fr, v.value));
            } else {// 'high'
                bm_max.push((v.fr, v.value));
            }
        }
    }
    let mut result = Vec::new();
    for (fr, sf_min) in sf_min {
        let sf_max = sf_max.iter().filter(|(fr_max, _)| fr == *fr_max).map(|(_, v)| *v ).next().expect("StrengthLimitDataArray convert error: sf_max");
        let bm_min = bm_min.iter().filter(|(fr_max, _)| fr == *fr_max).map(|(_, v)| *v ).next().expect("StrengthLimitDataArray convert error: bm_min");
        let bm_max = bm_max.iter().filter(|(fr_max, _)| fr == *fr_max).map(|(_, v)| *v ).next().expect("StrengthLimitDataArray convert error: bm_max");
        result.push((fr, bm_min, bm_max, sf_min, sf_max));
    }
    result
}
