//! Класс-коллекция таблиц. Проверяет данные и выполняет их запись
use crate::content::displacement::Displacement;
use crate::content::strength::Strength;
//use crate::content::general::General;
//use crate::content::list_of_calculations::ListOfCalculations;
use crate::content::Content;
use crate::error::Error;
//use crate::formatter::Page;
use crate::ApiServer;
use calamine::Range;
use calamine::{open_workbook, Data, Reader, Xlsx};
use std::collections::HashMap;
//
pub struct Report {
    ship_id: usize,
    api_server: ApiServer,
    general: HashMap<String, String>,
    ship_wide: Option<f64>,
    strength_target: Vec<(f64, i32, f64, f64)>, //x, fr, SF, BM
    strength_result: Vec<(f64, f64, f64)>, //x, SF, BM
    strength_limit: Vec<(f64, f64, f64, f64, f64)>, // fr, bm_min, bm_max, sf_min, sf_max
    lever_diagram_target: Vec<(f64, f64)>, //angle, level
    criteria_target: HashMap<i32, f64>,    //id, value
    parameters_target: HashMap<i32, f64>,  //id, value
    criterion_result: HashMap<i32, f64>,
    parameters_result: HashMap<i32, f64>,
}
//
impl Report {
    //
    pub fn new(ship_id: usize, api_server: ApiServer) -> Self {
        Self {
            ship_id,
            api_server,
            general: HashMap::new(),
            ship_wide: None,
            strength_target: Vec::new(),
            strength_result: Vec::new(),
            strength_limit: Vec::new(),
            lever_diagram_target: Vec::new(),
            criteria_target: HashMap::new(),
            parameters_target: HashMap::new(),
            criterion_result: HashMap::new(),
            parameters_result: HashMap::new(),
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
        self.general = Report::convert(workbook.get("General").ok_or(Error::FromString(format!(
            "Report get_target error: no table General!"
        )))?).iter().map(|v| (v[0].clone(), v[1].clone())).collect();
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
                ) {
                    (Ok(x), Ok(fr), Ok(sf), Ok(bm)) => Some((x, fr, sf, bm)),
                    _ => None, //Err(Error::FromString(format!("Report parse error: strength {:?}", v))),
                }
            })
            .collect();
        let lever_diagram = Report::convert(workbook.get("Stabilitycurve").ok_or(
            Error::FromString(format!("Report get_target error: no table Stabilitycurve!")),
        )?);
        self.lever_diagram_target = lever_diagram
            .iter()
            .filter_map(|v| {
                match (v[0].parse::<f64>(), v[1].parse::<f64>()) {
                    (Ok(a), Ok(l)) => Some((a, l)),
                    _ => None, //Err(Error::FromString(format!("Report parse error: lever_diagram {:?}", v))),
                }
            })
            .collect();
        let criteria =
            Report::convert(workbook.get("StabilityCriteria").ok_or(Error::FromString(
                format!("Report get_target error: no table StabilityCriteria!"),
            ))?);
        self.criteria_target = criteria
            .iter()
            .filter_map(|v| {
                match (v[0].parse::<i32>(), v[3].parse::<f64>()) {
                    (Ok(id), Ok(v)) => Some((id, v)),
                    _ => None, //Err(Error::FromString(format!("Report parse error: criteria {:?}", v))),
                }
            })
            .collect();
        let parameters = Report::convert(workbook.get("Parameters").ok_or(Error::FromString(
            format!("Report get_target error: no table Parameters!"),
        ))?);
        self.parameters_target = parameters
            .iter()
            .filter_map(|v| {
                match (v[0].parse::<i32>(), v[3].parse::<f64>()) {
                    (Ok(id), Ok(v)) => Some((id, v)),
                    _ => None, //Err(Error::FromString(format!("Report parse error: parameters {:?}", v))),
                }
            })
            .collect();
        Ok(())
    }
    //
    pub fn get_from_db(&mut self) -> Result<(), Error> {
        self.criterion_result =
            crate::db::api_server::get_criterion_data(&mut self.api_server, self.ship_id)?.data();
        self.parameters_result =
            crate::db::api_server::get_parameters_data(&mut self.api_server, self.ship_id)?.data();
        self.strength_result =
            crate::db::api_server::get_strength_result(&mut self.api_server, self.ship_id)?;
        let area = if self.general.get("Акватория").unwrap().contains("Море") {
            "sea"
        } else {
            "harbor"
        };
        self.strength_limit =
            crate::db::api_server::get_strength_limit(&mut self.api_server, self.ship_id, area)?;
        Ok(())
    }
    //
    pub fn get_ship_wide(&mut self) -> Result<(), Error> {
        self.ship_wide = crate::db::api_server::get_ship_wide(&mut self.api_server, self.ship_id)
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
        /*    let formatter = crate::formatter::Formatter::new(Page::new(Title::new(
                    "Сухогрузное судно Sofia (IMO№ 555666333)".to_owned(),
                ).print(), None)).add_page(General::new().to_string())
                .add_page(ListOfCalculations::new(vec!["Однородный груз. Отправление", "Дополнительные расчеты остойчивости, выполненные в связи с изменением и увеличением осадки. (2020)",
                "Судно в балласте. Прибытие", "Буклет остойчивости",
                "Тяжеловесный груз (20 т/м2). Отправление", "Дополнительные расчеты остойчивости, выполненные в связи с изменением и увеличением осадки. (2020)",
                "Контейнеры 12 т/ТЕU. Прибытие", "Буклет остойчивости",
                "Зерно 65 фут3/т. Прибытие", "Информация об остойчивости судна при перевозке зерна",]).to_string());
        let formatter = crate::formatter::Formatter::new(Page::new(
            Displacement::new(&self.parameters(&[2, 32, 56, 12, 1, 52]), self.ship_wide.unwrap()).to_string(),
            Some(1),
        ));
        std::fs::write(format!("{}", path), formatter.print()).expect("Unable to write {path}");
        */
        let mut content = Displacement::new(
            &self.parameters(&[2, 32, 56, 12, 1, 52]),
            self.ship_wide.unwrap(),
        )
        .to_string()?;
        content = content + "\n" + &Strength::new_named(&self.strength_result, &self.strength_target, &self.strength_limit).to_string()?;
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
            .map(|v| v.iter().map(|v| v.to_string()).collect())
            .collect();
        data
    }
    //
    fn parameter(&self, id: &i32) -> (Option<f64>, Option<f64>) {
        (
            self.parameters_target.get(id).copied(),
            self.parameters_result.get(id).copied(),
        )
    }
    //
    fn parameters(&self, id: &[i32]) -> Vec<(Option<f64>, Option<f64>)> {
        id.iter().map(|id| self.parameter(id)).collect()
    }
}
