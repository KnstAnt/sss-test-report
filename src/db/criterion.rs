use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::serde_parser::IFromJson;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataRow {
    pub id: i32,
    pub result: Option<f64>,
}
//
impl std::fmt::Display for DataRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DataRow(id:{}, result:{:?})",
            self.id, self.result
        )
    }
}
/// Массив данных по грузам
pub type DataRowArray = DataArray<DataRow>;
/// Массив ключ + значение
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataArray<T> {
    pub data: Vec<T>,
    pub error: HashMap<String, String>,
}
//
impl <T> IFromJson for DataArray<T> {
    fn error(&self) -> Option<&String> {
        self.error.values().next()
    }
}
//
impl DataArray<DataRow> {
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> HashMap<i32, f64> {
        self.data.iter().map(|v| (v.id, v.result.unwrap_or(0.)) ).collect()
    }
}