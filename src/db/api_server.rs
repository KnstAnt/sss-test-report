//! Функции для работы с АПИ-сервером
use crate::db::serde_parser::IFromJson;
use crate::error::Error;
use api_tools::client::api_query::*;
use api_tools::client::api_request::*;

use super::computed_frame::ComputedFrameDataArray;
use super::criterion::DataRowArray;
use super::criterion::DataShipArray;
use super::strength_result::StrengthResultDataArray;

pub struct ApiServer {
    database: String,
    request: Option<ApiRequest>,
}
//
impl ApiServer {
    pub fn new(database: String) -> Self {
        Self {
            database,
            request: None,
        }
    }
    //
    pub fn fetch(&mut self, sql: &str) -> Result<Vec<u8>, Error> {
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
            .map_err(|e| Error::FromString(format!("ApiServer fetch error: {e}")))
    }
}

/// Чтение данных из БД. Функция читает данные за несколько запросов,
/// парсит их и проверяет данные на корректность.
pub fn get_criterion_data(
    api_server: &mut ApiServer,
    ship_id: usize,
) -> Result<DataRowArray, Error> {
    DataRowArray::parse(
        &api_server
            .fetch(&format!(
                "SELECT criterion_id as id, actual_value as result FROM criterion_values WHERE ship_id={};",
                ship_id
            ))
            .map_err(|e| {
                Error::FromString(format!("api_server get_criterion_data error: {e}"))
            })?
    )
    .map_err(|e| Error::FromString(format!("api_server get_criterion_data error: {e}")))
}
//
pub fn get_parameters_data(
    api_server: &mut ApiServer,
    ship_id: usize,
) -> Result<DataRowArray, Error> {
    DataRowArray::parse(
        &api_server
            .fetch(&format!(
                "SELECT parameter_id as id, result FROM parameter_data WHERE ship_id={};",
                ship_id
            ))
            .map_err(|e| Error::FromString(format!("api_server get_parameters_data error: {e}")))?,
    )
    .map_err(|e| Error::FromString(format!("api_server get_parameters_data error: {e}")))
}
//
pub fn get_ship_wide(api_server: &mut ApiServer, ship_id: usize) -> Result<DataShipArray, Error> {
    DataShipArray::parse(
        &api_server
            .fetch(&format!(
                "SELECT key, value FROM ship_parameters WHERE key='MouldedBreadth' AND ship_id={};",
                ship_id
            ))
            .map_err(|e| Error::FromString(format!("api_server get_criterion_data error: {e}")))?,
    )
    .map_err(|e| Error::FromString(format!("api_server get_ship_wide error: {e}")))
}
//
pub fn get_strength_result(
    api_server: &mut ApiServer,
    ship_id: usize,
) -> Result<Vec<(f64, f64, f64)>, Error> {
    let bounds = ComputedFrameDataArray::parse(
        &api_server
            .fetch(&format!(
                "SELECT index, start_x, end_x FROM computed_frame_space WHERE ship_id={} ORDER BY index;",
                ship_id
            ))
            .map_err(|e| Error::FromString(format!("api_server get_strength_result bounds error: {e}")))?,
    )
    .map_err(|e| Error::FromString(format!("api_server get_strength_result bounds error: {e}")))?;
    let strength_result = StrengthResultDataArray::parse(
        &api_server
            .fetch(&format!(
                "SELECT value_shear_force as sf, value_bending_moment as bm FROM result_strength WHERE ship_id={} ORDER BY index;",
                ship_id
            ))
            .map_err(|e| Error::FromString(format!("api_server get_strength_result strength_result error: {e}")))?,
    )
    .map_err(|e| Error::FromString(format!("api_server get_strength_result strength_result error: {e}")))?;
    Ok(bounds
        .data()
        .iter()
        .zip(strength_result.data().iter())
        .map(|(x, (sf, bm))| (*x, *sf, *bm))
        .collect())
}
