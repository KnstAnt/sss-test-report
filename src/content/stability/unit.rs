use std::collections::HashMap;

use crate::error::Error;

//
#[derive(Debug, Clone)]
pub struct TableUnit {
    pub id: i32,
    pub name: String,
    pub unit: String,
    pub target: Option<f64>,
    pub result: Option<f64>,
    pub limit_percent: Option<String>,
    pub limit_abs: Option<String>,
}
//
impl TableUnit {
    //
    pub fn new(
        id: i32,
        name: String,
        unit: String,
        target: Option<f64>,
        result: Option<f64>,
        limit_percent: Option<String>,
        limit_abs: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            unit,
            target,
            result,
            limit_percent,
            limit_abs,
        }
    }
    //
    pub fn from_data(data: &[String], result: &HashMap<i32, f64>,) -> Result<Self, Error> {
        let id = data
            .get(0)
            .ok_or(Error::FromString(
                format!("TableUnit from_data error: no id!, {:?}",
                data,
            )))?
            .trim()
            .parse::<i32>()
            .map_err(|e| Error::FromString(
                format!("TableUnit from_data error: id!, data:{:?}, err:{e}",
                data,
            )))?;
        let name = data
            .get(1)
            .ok_or(Error::FromString(
               format!("TableUnit from_data error: no name!, {:?}",
                data,
            )))?
            .to_owned();
        let unit = data
            .get(2)
            .ok_or(Error::FromString(
                format!("TableUnit from_data error: no unit!, {:?}",
                data,
            )))?
            .to_owned();
        let target = data
            .get(3)
            .ok_or(Error::FromString(
                format!("TableUnit from_data error: no target!, {:?}",
                data,
            )))?
            .parse::<f64>()
            .ok();
        let result = result.get(&id).copied();
        let limit_percent = data.get(4).map_or(None, |s| Some(s.to_owned()) );
        let limit_abs = data.get(5).map_or(None, |s| Some(s.to_owned()) );
        Ok(Self::new(
            id,
            name,
            unit,
            target,
            result,
            limit_percent,
            limit_abs,
        ))
    }
}
