
use std::collections::HashMap;

use bulk_cargo::BulkCargo;
use bulkhead::Bulkhead;
use cargo::Cargo;
use container::Container;
use summary::Summary;
use tank::Tank;
use sal_core::{dbg::Dbg, error::Error};
use crate::{content::misc::lang::Lang, db::{bulk_cargo::BulkCargoData, bulkhead::BulkheadData, cargo::CargoData, container::ContainerData, parameters::ParameterData, tank::TankData}};

use super::Content;
pub mod tank;
pub mod cargo;
pub mod bulkhead;
pub mod bulk_cargo;
pub mod container;
pub mod summary;
mod table;
//
enum Title {
    Summary,
    BallastTanks,
    StoresTanks,
    Stores,
    Bulkheads,
    BulkCargo,
    Containers,
    GeneralCargo,
}
//
impl Title {
    pub fn val(&self, language: &Lang) -> String {
        match (self, language) {
            (Self::Summary, Lang::En) => "Total",
            (Self::Summary, Lang::Ru) => "Итого",
            (Self::BallastTanks, Lang::En) => "Ballast tanks",
            (Self::BallastTanks, Lang::Ru) => "Балластные цистерны",
            (Self::StoresTanks, Lang::En) => "Stores tanks",
            (Self::StoresTanks, Lang::Ru) => "Цистерны запаса",
            (Self::Stores, Lang::En) => "Stores",
            (Self::Stores, Lang::Ru) => "Запасы",
            (Self::Bulkheads, Lang::En) => "Bulkheads",
            (Self::Bulkheads, Lang::Ru) => "Зерновые переборки",
            (Self::BulkCargo, Lang::En) => "BulkCargo",
            (Self::BulkCargo, Lang::Ru) => "Навалочный груз",
            (Self::Containers, Lang::En) => "Containers",
            (Self::Containers, Lang::Ru) => "Контейнеры",
            (Self::GeneralCargo, Lang::En) => "GeneralCargo",
            (Self::GeneralCargo, Lang::Ru) => "Генеральный груз",
        }.to_owned()
    }
}
//
pub struct Displacement {
    dbg: Dbg,
    title: String,
    summary: Summary,
    ballast_tank: Tank,
    stores_tank: Tank,
    stores: Cargo,
    bulkhead: Bulkhead,
    bulk_cargo: BulkCargo,
    container: Container,
    general_cargo: Cargo,
}

//
impl Displacement {
    pub fn new(    
        dbg: Dbg,
        title: String,
        summary: Summary,
        ballast_tank: Tank,
        stores_tank: Tank,
        stores: Cargo,
        bulkhead: Bulkhead,
        bulk_cargo: BulkCargo,
        container: Container,
        general_cargo: Cargo,
    ) -> Self {
        Self {
            dbg,
            title,
            summary,
            ballast_tank,
            stores_tank,
            stores,
            bulkhead,
            bulk_cargo,
            container,
            general_cargo,
        }
    }
    //
    pub fn new_named(
        parent: &Dbg,
        language: &Lang,
        displacement_target: &Vec<Vec<String>>,
        parameters_result: &HashMap<i32, ParameterData>,
        ship_wide: f64,
        ballast_tanks: &[TankData],
        stores_tanks: &[TankData],
        stores: &[CargoData] ,
        bulkheads: &[BulkheadData] ,
        bulk_cargo: &[BulkCargoData],
        container: &[ContainerData],
        general_cargo: &[CargoData],        
    ) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Displacement");
        let title = if *language == Lang::En {
            "Displacement"
        } else {
            "Водоизмещение"
        }.to_owned();
        Ok(Self {
            dbg: dbg.clone(),
            title,
            summary: crate::content::displacement::summary::Summary::from(
                &dbg,
                Title::Summary.val(language),
                language,
                displacement_target,
                parameters_result,
                ship_wide,
            )?,
            ballast_tank: crate::content::displacement::tank::Tank::from(
                &dbg,
                Title::BallastTanks.val(language),
                language,
                ballast_tanks
            )?,
            stores_tank: crate::content::displacement::tank::Tank::from(
                &dbg,
                Title::StoresTanks.val(language),
                language,
                stores_tanks
            )?,
            stores: crate::content::displacement::cargo::Cargo::from(
                &dbg,
                Title::Stores.val(language),
                language,
                stores
            )?,
            bulkhead: crate::content::displacement::bulkhead::Bulkhead::from(
                &dbg,
                Title::Bulkheads.val(language),
                language,
                bulkheads
            )?,
            bulk_cargo: crate::content::displacement::bulk_cargo::BulkCargo::from(
                &dbg,
                Title::BulkCargo.val(language),
                language,
                bulk_cargo
            )?,
            container: crate::content::displacement::container::Container::from(
                &dbg,
                Title::Containers.val(language),
                language,
                container
            )?,
            general_cargo: crate::content::displacement::cargo::Cargo::from(
                &dbg,
                Title::GeneralCargo.val(language),
                language,
                general_cargo
            )?            
        })
    }    
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok("# ".to_string() + &self.title + "\n\n" +
            "## " + &self.summary.to_string()? + "\n" +
            "## " + &self.ballast_tank.to_string()? + "\n" +
            "## " + &self.stores_tank.to_string()? + "\n" +
            "## " + &self.stores.to_string()? + "\n" +
            "## " + &self.bulkhead.to_string()? + "\n" +
            "## " + &self.bulk_cargo.to_string()? + "\n" +
            "## " + &self.container.to_string()? + "\n" +
            "## " + &self.general_cargo.to_string()? + "\n"
        )
    }
}
