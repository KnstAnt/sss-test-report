//! Функции для работы с БД
use crate::db::serde_parser::IFromJson;
use sal_core::{dbg::Dbg, error::Error};
use api_tools::client::api_query::*;
use api_tools::client::api_request::*;

use super::bulk_cargo::BulkCargoDataArray;
use super::bulkhead::BulkheadDataArray;
use super::cargo::CargoDataArray;
use super::container::ContainerDataArray;
use super::criterion::CriteriaDataArray;
use super::data::DataRowArray;
use super::data::DataShipArray;
use super::parameters::ParameterDataArray;
use super::stability_diagram::StabilityDiagramDataArray;
use super::strength_result::StrengthResultDataArray;
use super::tank::TankDataArray;

mod client;
pub(crate) use client::*;

pub struct Db {
    dbg: Dbg,
    ship_id: String,
    project_id: String,
    language: String,
    api_client: ApiClient,
}
//
impl Db {
    pub fn new(
        parent: &Dbg,
        ship_id: String,
        project_id: String,
        language: Option<String>, // "ru" - russian (default) / "en" - english
        api_client: ApiClient,        
    ) -> Self {
        let dbg = Dbg::new(parent, "ModelCached");
        Self {
            dbg,
            ship_id,
            project_id,
            language: language
                .map_or("ru", |v| if v.contains("en") { "en" } else { "ru" })
                .to_owned(),
            api_client,
        }
    }
    /// Чтение данных из БД. Функция читает данные за несколько запросов,
    /// парсит их и проверяет данные на корректность.
    /// Чтение данных из БД. Функция читает данные за несколько запросов,
    /// парсит их и проверяет данные на корректность.
    pub fn get_criterion_data(&mut self) -> Result<CriteriaDataArray, Error> {
        let error = Error::new(&self.dbg, "get_criterion_data");
        CriteriaDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                "SELECT 
                    id AS id, \
                    title as name, \
                    unit as unit, \
                    result AS result, \
                    target AS target, \
                    state as state
                FROM 
                    criterion_view
                WHERE 
                    language={} AND
                    category_id = 1 AND
                    ship_id={} AND 
                    project_id IS NOT DISTINCT FROM {}
                ORDER BY
                    id;",
                    self.language,
                    self.ship_id, 
                    self.project_id,
                ))
                .map_err(|e| {
                    error.pass(e)
                })?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_parameters_data(&mut self) -> Result<ParameterDataArray, Error> {
        let error = Error::new(&self.dbg, "get_parameters_data");
        ParameterDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                "SELECT 
                    id as id, \
                    title as name, \
                    result as result, \
                    unit as unit
                FROM 
                    parameter_view
                WHERE 
                    language={} AND
                    ship_id={} AND 
                    project_id IS NOT DISTINCT FROM {}
                ORDER BY
                    id;",
                    self.language,
                    self.ship_id, 
                    self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_ship_wide(&mut self) -> Result<DataShipArray, Error> {
        let error = Error::new(&self.dbg, "get_ship_wide");
        DataShipArray::parse(
            &self
                .api_client
                .fetch(&format!(
                "SELECT 
                  key, \
                  value
                FROM 
                  \"ship/ship_general_characteristics\"
                WHERE
                  key='MouldedBreadth' 
                  AND ship_id={} 
                  AND project_id IS NOT DISTINCT FROM {}",
                self.ship_id,
                self.project_id,
            ))
                .map_err(|e| {
                    error.pass(e)
                })?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_strength_result(&mut self) -> Result<StrengthResultDataArray, Error> {
        let error = Error::new(&self.dbg, "get_strength_result");
        let strength_result = StrengthResultDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                        frame_x as x, \
                        value_shear_force as sf, \
                        value_bending_moment as bm, \
                        limit_low_shear_force as sf_limit_low, \
                        limit_high_shear_force as sf_limit_high, \
                        percent_shear_force as sf_percent, \
                        status_shear_force as sf_status, \
                        limit_low_bending_moment as bm_limit_low, \
                        limit_high_bending_moment as bm_limit_high, \
                        percent_bending_moment as bm_percent, \
                        status_bending_moment as bm_status
                    FROM
                        result_strength_force_and_moment
                    WHERE 
                        ship_id={} AND
                        project_id IS NOT DISTINCT FROM {}
                    ORDER BY x;",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass( e))?,
        )
        .map_err(|e| error.pass(e))?;
        Ok(strength_result)
    }
    //
    pub fn get_lever_diagram(&mut self) -> Result<Vec<(f64, f64)>, Error> {
        let error = Error::new(&self.dbg, "get_lever_diagram");
        Ok(StabilityDiagramDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                      angle, \
                      value_dso 
                    FROM 
                      stability_diagram 
                    WHERE 
                      ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| {
                    error.pass( e)
                })?,
        )
        .map_err(|e| error.pass(e))?
        .data())
    }

    pub fn get_ballast_tanks(&mut self) -> Result<TankDataArray, Error> {
        let error = Error::new(&self.dbg, "get_ballast_tanks");
        TankDataArray::parse(
            &self
                .api_client
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
                    self.language,
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| {
                    error.pass(e)
                })?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_stores_tanks(&mut self) -> Result<TankDataArray, Error> {
        let error = Error::new(&self.dbg, "get_stores_tanks");
        TankDataArray::parse(
            &self.api_client
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
                        category_id>=3 AND category_id<=8 AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language,
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_stores(&mut self) -> Result<CargoDataArray, Error> {
        let error = Error::new(&self.dbg, "get_stores");
        CargoDataArray::parse(
            &self
                .api_client
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
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_bulkheads(&mut self) -> Result<BulkheadDataArray, Error> {
        let error = Error::new(&self.dbg, "get_bulkheads");
        BulkheadDataArray::parse(
            &self
                .api_client
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
                            self.language,
                            self.language,
                            self.ship_id,
                            self.project_id,
                        ))
                        .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_bulk_cargo(&mut self) -> Result<BulkCargoDataArray, Error> {
        let error = Error::new(&self.dbg, "get_bulk_cargo");
        BulkCargoDataArray::parse(
            &self
                .api_client
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
                    self.language,
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_container(&mut self) -> Result<ContainerDataArray, Error> {
        let error = Error::new(&self.dbg, "get_container");
        ContainerDataArray::parse(
            &self
                .api_client
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
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_general_cargo(&mut self) -> Result<CargoDataArray, Error> {
        let error = Error::new(&self.dbg, "get_container");
        CargoDataArray::parse(
            &self
                .api_client
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
                    error.pass(e)
                })?,
        )
        .map_err(|e| error.pass(e))
    }
}
