//! Класс-коллекция таблиц. Проверяет данные и выполняет их запись
use crate::content::stability::Stability;
use crate::content::strength::Strength;
//use crate::content::general::General;
//use crate::content::list_of_calculations::ListOfCalculations;
use crate::content::Content;
use crate::db::bulk_cargo::BulkCargoData;
use crate::db::bulkhead::BulkheadData;
use crate::db::cargo::CargoData;
use crate::db::container::ContainerData;
use crate::db::criterion::CriteriaData;
use crate::db::parameters::ParameterData;
use crate::db::tank::TankData;
use crate::error::Error;
//use crate::formatter::Page;
use crate::ApiServer;
use calamine::Range;
use calamine::{open_workbook, Data, Reader, Xlsx};
use std::collections::HashMap;
//
pub struct Report {
    language: String,
    ship_id: usize,
    api_server: ApiServer,
    general: HashMap<String, String>,
    ship_wide: Option<f64>,
    strength_target: Vec<(f64, i32, f64, f64, f64)>, //x, fr, SF, BM, limit_%
    strength_target_max: Vec<(String, f64, f64, f64)>, // name, x, value, limit_%
    strength_result: Vec<(f64, f64, f64)>,           //x, SF, BM
    strength_limit: Vec<(f64, f64, f64, f64, f64)>,  // fr, bm_min, bm_max, sf_min, sf_max
    lever_diagram_result: Vec<(f64, f64)>,           //angle, level
    lever_diagram_target: Vec<(f64, f64, f64, f64)>, //angle, level, limit_%, limit_abs
    criteria_target: Vec<Vec<String>>,
    displacement_target: Vec<Vec<String>>,
    draught_target: Vec<Vec<String>>,
    parameters_target: Vec<Vec<String>>,
    criteria_result: HashMap<i32, CriteriaData>, // criterion_id, value
    parameters_result: HashMap<i32, ParameterData>,// parameter_id, value
    ballast_tanks: Vec<TankData>,
    stores_tanks: Vec<TankData>,
    stores: Vec<CargoData>,
    bulkheads: Vec<BulkheadData>,
    bulk_cargo: Vec<BulkCargoData>,
    container: Vec<ContainerData>,
    general_cargo: Vec<CargoData>,
}
//
impl Report {
    //
    pub fn new(language: Option<String>, ship_id: usize, api_server: ApiServer) -> Self {
        Self {
            language: language.unwrap_or("ru".to_owned()),
            ship_id,
            api_server,
            general: HashMap::new(),
            ship_wide: None,
            strength_target: Vec::new(),
            strength_target_max: Vec::new(),
            strength_result: Vec::new(),
            strength_limit: Vec::new(),
            lever_diagram_result: Vec::new(),
            lever_diagram_target: Vec::new(),
            criteria_target: Vec::new(),
            displacement_target: Vec::new(),
            draught_target: Vec::new(),
            parameters_target: Vec::new(),
            criteria_result: HashMap::new(),
            parameters_result: HashMap::new(),
            ballast_tanks: Vec::new(),
            stores_tanks: Vec::new(),
            stores: Vec::new(),
            bulkheads: Vec::new(),
            bulk_cargo: Vec::new(),
            container: Vec::new(),
            general_cargo: Vec::new(),
        }
    }
    //
    pub fn get_target(&mut self, path: &str) -> Result<(), Error> {
        let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
        let workbook: HashMap<String, Range<Data>> = workbook
            .worksheets()
            .into_iter()
            .filter(|(_, range)| range.used_cells().count() > 0)
            .collect();
        self.general = Report::convert(workbook.get("General").ok_or(Error::FromString(
            format!("Report get_target error: no table General!"),
        ))?)
        .iter()
        .map(|v| (v[0].clone(), v[1].clone()))
        .collect();
        let strength = Report::convert(workbook.get("SF&BM").ok_or(Error::FromString(format!(
            "Report get_target error: no table SF&BM!"
        )))?);
        self.strength_target = strength
            .iter()
            .filter_map(|v| {
                match (
                    v[0].parse::<f64>(),
                    v[1].parse::<i32>(),
                    v[2].parse::<f64>(),
                    v[3].parse::<f64>(),
                    v[4].parse::<f64>(),
                ) {
                    (Ok(x), Ok(fr), Ok(sf), Ok(bm), Ok(limit_p)) => Some((x, fr, sf, bm, limit_p)),
                    _ => None, //Err(Error::FromString(format!("Report parse error: strength {:?}", v))),
                }
            })
            .collect();
        self.strength_target_max = if let Some(data) = workbook.get("SF&BM_max") {
            Report::convert(data)
                .iter()
                .filter_map(|v| {
                    match (
                        v[0].to_owned(),
                        v[1].parse::<f64>(),
                        v[2].parse::<f64>(),
                        v[3].parse::<f64>(),
                    ) {
                        (name, Ok(x), Ok(value), Ok(limit_p)) => Some((name, x, value, limit_p)),
                        _ => None, //Err(Error::FromString(format!("Report parse error: strength_max {:?}", v))),
                    }
                })
                .collect()
        } else {
            Vec::new()
        };
        let lever_diagram = Report::convert(workbook.get("Stabilitycurve").ok_or(
            Error::FromString(format!("Report get_target error: no table Stabilitycurve!")),
        )?);
        self.lever_diagram_target = lever_diagram
            .iter()
            .filter_map(|v| {
                match (
                    v[0].parse::<f64>(),
                    v[1].parse::<f64>(),
                    v[2].parse::<f64>(),
                    v[3].parse::<f64>(),
                ) {
                    (Ok(a), Ok(l), Ok(limit_p), Ok(limit_abs)) => Some((a, l, limit_p, limit_abs)),
                    _ => None, //Err(Error::FromString(format!("Report parse error: lever_diagram {:?}", v))),
                }
            })
            .collect();
        self.criteria_target = Report::convert(workbook.get("StabilityCriteria").ok_or(Error::FromString(
            format!("Report get_target error: no table StabilityCriteria!"),
        ))?);
        let parameters = Report::convert(workbook.get("Parameters").ok_or(Error::FromString(
            format!("Report get_target error: no table Parameters!"),
        ))?);
        let mut buf = Vec::new();
        for row in parameters.into_iter().rev() {
            if row[0].contains("Водоизмещение") {
                buf.reverse();
                self.displacement_target = buf;
        //        dbg!(&self.displacement_target);
                buf = Vec::new();
                continue;
            }
            if row[0].contains("Осадки") {
                buf.reverse();
                self.draught_target = buf;
        //        dbg!(&self.draught_target);
                buf = Vec::new();
                continue;
            }
            if row[0].contains("Остойчивость") {
                buf.reverse();
                self.parameters_target = buf;
        //        dbg!(&self.parameters_target);
                buf = Vec::new();
                continue;
            }
            if row.len() > 1 {
                buf.push(row);
            }
        }
        Ok(())
    }
    //
    pub fn get_from_db(&mut self) -> Result<(), Error> {
        self.criteria_result =
            self.api_server.get_criterion_data()?.data().into_iter().map(|v| 
                // Если ид=17 - Минимальная метацентрическая высота деления на отсеки
                // то для отчета берем целевое значение
                if v.0 != 17 {
                    (v.0, v.1)
                } else {
                    let mut data = v.1;
                    data.result = data.target;
                    (v.0, data)
                }).collect();
        self.parameters_result =
            self.api_server.get_parameters_data()?.data().into_iter().map(|v| 
                    (v.0, v.1)
        ).collect();
        self.strength_result =
            self.api_server.get_strength_result()?;
        let area = if self.general.get("Акватория").unwrap().contains("Море") {
            "sea"
        } else {
            "harbor"
        };
        self.strength_limit =
            self.api_server.get_strength_limit(area)?;
        self.lever_diagram_result =
            self.api_server.get_lever_diagram()?;
        self.ballast_tanks =
            self.api_server.get_ballast_tanks()?.data();
        self.stores_tanks =
            self.api_server.get_stores_tanks()?.data();
        self.stores = self.api_server.get_stores()?.data();
        self.bulkheads =
            self.api_server.get_bulkheads()?.data();
        self.bulk_cargo =
            self.api_server.get_bulk_cargo()?.data();
        self.container =
            self.api_server.get_container()?.data();
        self.general_cargo =
            self.api_server.get_general_cargo()?.data();
        Ok(())
    }
    //   
    pub fn get_ship_wide(&mut self) -> Result<(), Error> {
        self.ship_wide = self.api_server.get_ship_wide()
            .map_err(|e| format!("Parser get_ship_wide error: {e}"))?
            .data()
            .get("MouldedBreadth")
            .copied();
        if self.ship_wide.is_none() || self.ship_wide.unwrap() <= 0. {
            return Err(Error::FromString(format!(
                "Parser get_ship_wide error: ship_wide {:?}",
                self.ship_wide
            )));
        }
        Ok(())
    }
    //
    pub fn write(self, path: &str) -> Result<(), Error> {
        println!("Parser write_to_file begin");
    //    dbg!(&self.parameters_target);
        let mut content = crate::content::displacement::Displacement::new(
            &self.language,            
            crate::content::displacement::summary::Summary::from(
                &self.language,
                &self.displacement_target,
                &self.parameters_result,
                self.ship_wide.unwrap(),
            )?,
            crate::content::displacement::tank::Tank::from(
                &self.language,
                &self.ballast_tanks
            )?,
            crate::content::displacement::tank::Tank::from(
                &self.language,
                &self.stores_tanks
            )?,
            crate::content::displacement::cargo::Cargo::from(
                &self.language,
                &self.stores
            )?,
            crate::content::displacement::bulkhead::Bulkhead::from(
                &self.language,
                &self.bulkheads
            )?,
            crate::content::displacement::bulk_cargo::BulkCargo::from(
                &self.language,
                &self.bulk_cargo
            )?,
            crate::content::displacement::container::Container::from(
                &self.language,
                &self.container
            )?,
            crate::content::displacement::cargo::Cargo::from(
                &self.language,
                &self.general_cargo
            )?,
        )
        .to_string()?;
        content += &(crate::content::stability::draught::Draught::from(
            &self.language,
            &self.draught_target,
            &self.parameters_result,
            self.ship_wide.unwrap(),
        )?.to_string().map_err(|e| format!("Parser write Draught error:{}", e))? + "\n");        
        content += &(Strength::new_named(
            &self.language,
            &self.strength_result,
            &self.strength_target,
            &self.strength_target_max,
            &self.strength_limit,
        ).to_string().map_err(|e| format!("Parser write Strength error:{}", e))? + "\n"); 
        content += &(Stability::new_named(
            &self.language,
            &self.criteria_target,
            &self.criteria_result,
            &self.parameters_target,
            &self.parameters_result,
            self.ship_wide.unwrap(),
            &self.lever_diagram_target,
            &self.lever_diagram_result,
        )?.to_string().map_err(|e| format!("Parser write Stability error:{}", e))? + "\n"); 
        std::fs::write(format!("{}", path), content).expect("Unable to write {path}");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Parser write_to_file end");
        Ok(())
    }
    //
    fn convert(data: &Range<Data>) -> Vec<Vec<String>> {
        let data: Vec<&[Data]> = data.rows().filter(|v| !v.is_empty()).collect();
        let data = data
            .iter()
            .map(|v| v.iter().map(|v| v.to_string().replace(" \u{a0}", "")).collect())
            .collect();
        data
    }
}
