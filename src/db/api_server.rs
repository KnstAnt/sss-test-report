//! Функции для работы с АПИ-сервером
use crate::error::Error;
use crate::db::serde_parser::IFromJson;
use api_tools::client::api_query::*;
use api_tools::client::api_request::*;

use super::criterion::DataRowArray;
use super::criterion::DataShipArray;

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
        request.fetch(true).map_err(|e| Error::FromString(format!("ApiServer fetch error: {e}")))
    }
}

/// Чтение данных из БД. Функция читает данные за несколько запросов,
/// парсит их и проверяет данные на корректность.
pub fn get_criterion_data(api_server: &mut ApiServer, ship_id: usize) -> Result<DataRowArray, Error> {
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
pub fn get_parameters_data(api_server: &mut ApiServer, ship_id: usize) -> Result<DataRowArray, Error> {
    DataRowArray::parse(
        &api_server
            .fetch(&format!(
                "SELECT parameter_id as id, result FROM parameter_data WHERE ship_id={};",
                ship_id
            ))
            .map_err(|e| {
                Error::FromString(format!("api_server get_parameters_data error: {e}"))
            })?
    )
    .map_err(|e| Error::FromString(format!("api_server get_parameters_data error: {e}")))
}
//
pub fn get_ship_wide(api_server: &mut ApiServer, ship_id: usize) -> Result<DataShipArray, Error> {
    DataShipArray::parse(
        &api_server
            .fetch(&format!(
                "SELECT value FROM ship_parameters WHERE key='MouldedBreadth' AND ship_id={};",
                ship_id
            ))
            .map_err(|e| {
                Error::FromString(format!("api_server get_criterion_data error: {e}"))
            })?
    )
    .map_err(|e| Error::FromString(format!("api_server get_ship_wide error: {e}")))
}
