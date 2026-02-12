use std::collections::HashMap;

use sal_core::{dbg::Dbg, error::Error};

use crate::{db::{criterion::CriteriaData, parameters::ParameterData}};

//
#[derive(Debug, Clone)]
pub struct TableUnit {
    dbg: Dbg,
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
        parent: &Dbg,
        id: i32,
        name: String,
        unit: String,
        target: Option<f64>,
        result: Option<f64>,
        limit_percent: Option<String>,
        limit_abs: Option<String>,
    ) -> Self {
        let dbg = Dbg::new(parent, "TableUnit");
        Self {
            dbg,
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
    pub fn from_parameters(
        parent: &Dbg, 
        data: &[String], 
        result: &HashMap<i32, 
        ParameterData>
    ) -> Result<Self, Error> {
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
        let target = data
            .get(3)
            .ok_or(Error::FromString(
                format!("TableUnit from_data error: no target!, {:?}",
                data,
            )))?
            .parse::<f64>()
            .ok(); 
        let limit_percent = data.get(4).map_or(None, |s| Some(s.to_owned()) );
        let limit_abs = data.get(5).map_or(None, |s| Some(s.to_owned()) );
        let (name, unit, result) = if let Some(data) = result.get(&id) {
            let name = data.name.clone().unwrap_or("".to_owned());
            let unit = data.unit.clone().unwrap_or("".to_owned());
            let result = data.result.clone();
            (name, unit, result)
        } else {
            return Err(Error::FromString(format!("TableUnit from_parameters error: no data!")));
        };
        Ok(Self::new(
            parent,
            id,
            name,
            unit,
            target,
            result,
            limit_percent,
            limit_abs,
        ))
    }
    //
    pub fn from_criterion(
        parent: &Dbg, 
        data: &[String], 
        result: &HashMap<i32, CriteriaData>,
    ) -> Result<Self, Error> {
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
        let target = data
            .get(3)
            .ok_or(Error::FromString(
                format!("TableUnit from_data error: no target!, {:?}",
                data,
            )))?
            .parse::<f64>()
            .ok(); 
        let limit_percent = data.get(4).map_or(None, |s| Some(s.to_owned()) );
        let limit_abs = data.get(5).map_or(None, |s| Some(s.to_owned()) );
        let (name, unit, result) = if let Some(data) = result.get(&id) {
            let name = data.name.clone();
            let unit = data.unit.clone().unwrap_or("".to_owned());
            let result = data.result.clone();
            (name, unit, result)
        } else {
            return Err(Error::FromString(format!("TableUnit from_criterion error: no data!")));
        };
        Ok(Self::new(
            parent,
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
