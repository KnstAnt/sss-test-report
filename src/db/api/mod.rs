//! Функции для работы с БД
use crate::db::serde_parser::IFromJson;
use sal_core::{dbg::Dbg, error::Error};
use api_tools::client::api_query::*;
use api_tools::client::api_request::*;

use super::bulk_cargo::BulkCargoDataArray;
use super::bulkhead::BulkheadDataArray;
use super::cargo::CargoDataArray;
use super::computed_frame::ComputedFrameDataArray;
use super::container::ContainerDataArray;
use super::criterion::CriteriaDataArray;
use super::data::DataRowArray;
use super::data::DataShipArray;
use super::parameters::ParameterDataArray;
use super::stability_diagram::StabilityDiagramDataArray;
use super::strength_limit::StrengthLimitDataArray;
use super::strength_result::StrengthResultDataArray;
use super::tank::TankDataArray;

mod client;
pub(crate) use client::*;

pub struct Db {
    dbg: Dbg,
    language: String,
    api_client: ApiClient,
}
//
impl Db {
    pub fn new(
        parent: &Dbg,
        api_client: ApiClient,
        language: Option<String>, // "ru" - russian (default) / "en" - english
    ) -> Self {
        let dbg = Dbg::new(parent, "ModelCached");
        Self {
            dbg,
            language: language
                .map_or("ru", |v| if v.contains("en") { "en" } else { "ru" })
                .to_owned(),
            api_client,
        }
    }
    //
    fn language<'a>(&self, ru: &'a str, en: &'a str) -> &'a str {
        if self.language.contains("en") {
            en
        } else {
            ru
        }
    }
    //
    pub fn fetch(&mut self, sql: &str) -> Result<Vec<u8>, Error> {
        let error = Error::new(&self.dbg, "fetch");
        let mut request = ApiRequest::new(
            &api_tools::debug::dbg_id::DbgId("parent".to_owned()),
            "0.0.0.0:8080",
            "auth_token",
            ApiQuery::new(
                ApiQueryKind::Sql(ApiQuerySql::new(self.database.clone(), sql)),
                false,
            ),
            true,
            false,
        );
        request
            .fetch(true)
            .map_err(|e| Error::FromString(format!("Db fetch error: {e}")))
    }
    /// Чтение данных из БД. Функция читает данные за несколько запросов,
    /// парсит их и проверяет данные на корректность.
    /// Чтение данных из БД. Функция читает данные за несколько запросов,
    /// парсит их и проверяет данные на корректность.
    pub fn get_criterion_data(&mut self) -> Result<CriteriaDataArray, Error> {
        let error = Error::new(&self.dbg, "get_criterion_data");
        CriteriaDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    head.id AS id, \
                    head.{} as name, \
                    unit.{} as unit, \
                    values.actual_value AS result, \
                    values.limit_value AS target, \
                    values.state as state
                FROM 
                    criterion as head
                LEFT JOIN
                    unit as unit on head.unit_id=unit.id
                RIGHT JOIN
                    criterion_values AS values ON head.id=values.criterion_id
                WHERE 
                    values.ship_id={} AND 
                    head.category_id = 1 AND
                    values.project_id IS NOT DISTINCT FROM {}
                ORDER BY
                    head.id;",
                    self.language("title_rus", "title_eng"),
                    self.language("symbol_rus", "symbol_eng"),
                    self.ship_id, 
                    self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_client get_criterion_data error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_criterion_data error: {e}")))
    }
    //
    pub fn get_parameters_data(&mut self) -> Result<ParameterDataArray, Error> {
        let error = Error::new(&self.dbg, "get_parameters_data");
        ParameterDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    head.id as id, \
                    head.{} as name, \
                    data.result as result, \
                    unit.{} as unit
                FROM 
                    parameter_head as head
                LEFT JOIN
                    unit as unit on head.unit_id=unit.id                    
                RIGHT JOIN                
                    parameter_data as data on data.parameter_id=head.id
                WHERE 
                    ship_id={} AND project_id IS NOT DISTINCT FROM {}
                ORDER BY
                    head.id;",
                    self.language("title_rus", "title_eng"),
                    self.language("symbol_rus", "symbol_eng"),
                    self.ship_id, 
                    self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_client get_parameters_data error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_parameters_data error: {e}")))
    }
    //
    pub fn get_ship_wide(&mut self) -> Result<DataShipArray, Error> {
        let error = Error::new(&self.dbg, "get_ship_wide");
        DataShipArray::parse(
            &self
                .fetch(&format!(
                "SELECT key, value FROM ship_parameters WHERE key='MouldedBreadth' AND ship_id={} AND project_id IS NOT DISTINCT FROM {}",
                self.ship_id,
                self.project_id,
            ))
                .map_err(|e| {
                    Error::FromString(format!("api_client get_criterion_data error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_ship_wide error: {e}")))
    }
    //
    pub fn get_strength_result(&mut self) -> Result<Vec<(f64, f64, f64)>, Error> {
        let error = Error::new(&self.dbg, "get_strength_result");
        let bounds = ComputedFrameDataArray::parse(
            &self
            .fetch(&format!(
                "SELECT index, start_x, end_x FROM computed_frame_space WHERE ship_id={} AND project_id IS NOT DISTINCT FROM {} ORDER BY index;",
                self.ship_id,
                self.project_id,
            ))
            .map_err(|e| Error::FromString(format!("api_client get_strength_result bounds error: {e}")))?,
    )
    .map_err(|e| Error::FromString(format!("api_client get_strength_result bounds error: {e}")))?;
        let strength_result = StrengthResultDataArray::parse(
            &self
            .fetch(&format!(
                "SELECT value_shear_force as sf, value_bending_moment as bm FROM result_strength WHERE ship_id={} AND project_id IS NOT DISTINCT FROM {} ORDER BY index;",
                self.ship_id,
                self.project_id,
            ))
            .map_err(|e| Error::FromString(format!("api_client get_strength_result strength_result error: {e}")))?,
    )
    .map_err(|e| Error::FromString(format!("api_client get_strength_result strength_result error: {e}")))?;
        Ok(bounds
            .data()
            .iter()
            .zip(strength_result.data().iter())
            .map(|(x, (sf, bm))| (*x, *sf, *bm))
            .collect())
    }
    // (frame_x, bm_min, bm_max, sf_min, sf_max)
    pub fn get_strength_limit(
        &mut self,
        area: &str,
    ) -> Result<Vec<(f64, f64, f64, f64, f64)>, Error> {
        let error = Error::new(&self.dbg, "get_strength_limit");
        Ok(StrengthLimitDataArray::parse(
            &self
            .fetch(&format!(
                "SELECT frame_x, value, limit_type::TEXT, limit_area::TEXT, force_type::TEXT FROM strength_force_limit WHERE ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                self.ship_id,
                self.project_id,
            ))
            .map_err(|e| Error::FromString(format!("api_client get_strength_limit error: {e}")))?,
    )
    .map_err(|e| Error::FromString(format!("api_client get_strength_limit error: {e}")))?.data(area))
    }
    //
    pub fn get_lever_diagram(&mut self) -> Result<Vec<(f64, f64)>, Error> {
        let error = Error::new(&self.dbg, "get_lever_diagram");
        Ok(StabilityDiagramDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT angle, value_dso FROM stability_diagram WHERE ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_client get_lever_diagram error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_lever_diagram error: {e}")))?
        .data())
    }

    pub fn get_ballast_tanks(&mut self) -> Result<TankDataArray, Error> {
        let error = Error::new(&self.dbg, "get_ballast_tanks");
        TankDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT 
                        {} as name, \
                        mass, \
                        mass_shift_x as x_g, \
                        mass_shift_y as y_g, \
                        mass_shift_z as z_g, \
                        m_f_s_x as f_sx 
                    FROM 
                        compartment 
                    WHERE 
                        category_id=2 AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language("name_rus", "name_engl"),
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_client get_ballast_tanks error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_ballast_tanks error: {e}")))
    }
    //
    pub fn get_stores_tanks(&mut self) -> Result<TankDataArray, Error> {
        let error = Error::new(&self.dbg, "get_stores_tanks");
        TankDataArray::parse(
            &self.fetch(&format!(
                    "SELECT 
                        {} as name, \
                        mass, \
                        mass_shift_x as x_g, \
                        mass_shift_y as y_g, \
                        mass_shift_z as z_g, \
                        m_f_s_x as f_sx 
                    FROM 
                        compartment 
                    WHERE 
                        category_id>=3 AND category_id<=8 AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language("name_rus", "name_engl"),
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_client get_stores_tanks error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_stores_tanks error: {e}")))
    }
    //
    pub fn get_stores(&mut self) -> Result<CargoDataArray, Error> {
        let error = Error::new(&self.dbg, "get_stores");
        CargoDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT 
                        name as name, \
                        mass, \
                        mass_shift_x as x_g, \
                        mass_shift_y as y_g, \
                        mass_shift_z as z_g
                    FROM 
                        cargo 
                    WHERE 
                        category_id=9 AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_client get_stores error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_stores error: {e}")))
    }
    //
    pub fn get_bulkheads(&mut self) -> Result<BulkheadDataArray, Error> {
        let error = Error::new(&self.dbg, "get_bulkheads");
        BulkheadDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT 
                        b.{} as name, \
                        bp.{} as position, \
                        b.mass, \
                        bp.mass_shift_x as x_g, \
                        bp.mass_shift_y as y_g, \
                        bp.mass_shift_z as z_g
                    FROM 
                        bulkhead as b
                    JOIN 
                        bulkhead_place as bp ON b.id = bp.bulkhead_id
                    WHERE 
                        b.ship_id={} AND b.project_id IS NOT DISTINCT FROM {};",
                            self.language("name_rus", "name_engl"),
                            self.language("name_rus", "name_engl"),
                            self.ship_id,
                            self.project_id,
                        ))
                        .map_err(|e| Error::FromString(format!("api_client get_bulkheads error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_bulkheads error: {e}")))
    }
    //
    pub fn get_bulk_cargo(&mut self) -> Result<BulkCargoDataArray, Error> {
        let error = Error::new(&self.dbg, "get_bulk_cargo");
        BulkCargoDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT 
                        {} as name, \
                        mass, \
                        mass_shift_x as x_g, \
                        mass_shift_y as y_g, \
                        mass_shift_z as z_g, \
                        grain_moment
                    FROM 
                        hold_compartment 
                    WHERE 
                        ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language("name_rus", "name_engl"),
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_client get_bulk_cargo error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_bulk_cargo error: {e}")))
    }
    //
    pub fn get_container(&mut self) -> Result<ContainerDataArray, Error> {
        let error = Error::new(&self.dbg, "get_container");
        ContainerDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT 
                        c.owner_code as owner_code, \
                        c.serial_code as serial_code, \
                        c.check_digit, \
                        cs.bay_number as bay_number, \
                        cs.row_number as row_number, \
                        cs.tier_number as tier_number, \
                        c.gross_mass as mass, \
                        (cs.bound_x1 + (cs.bound_x2 - cs.bound_x1) / 2) AS x_g, \
                        (cs.bound_y1 + (cs.bound_y2 - cs.bound_y1) / 2) AS y_g, \
                        (cs.bound_z1 + (cs.bound_z2 - cs.bound_z1) / 2) AS z_g
                    FROM 
                        container as c
                    JOIN 
                        container_slot as cs ON cs.container_id = c.id
                    WHERE 
                        c.ship_id={} AND c.project_id IS NOT DISTINCT FROM {};",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_client get_container error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_container error: {e}")))
    }
    //
    pub fn get_general_cargo(&mut self) -> Result<CargoDataArray, Error> {
        let error = Error::new(&self.dbg, "get_container");
        CargoDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT 
                        name as name, \
                        mass, \
                        mass_shift_x as x_g, \
                        mass_shift_y as y_g, \
                        mass_shift_z as z_g
                    FROM 
                        cargo 
                    WHERE 
                        category_id=14 AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_client get_general_cargo error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_client get_general_cargo error: {e}")))
    }
}
