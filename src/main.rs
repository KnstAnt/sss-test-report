use crate::conf::conf::Conf;
use crate::db::api::ApiClient;
use debugging::session::debug_session::{DebugSession, LogLevel};
use log::info;
use parser::Report;
use sal_core::dbg::Dbg;
use std::io;
use std::io::*;

mod conf;
mod content;
mod db;
mod formatter;
mod parser;

fn main() {
    DebugSession::new()
        .filter(LogLevel::Trace)
        .module("api_tools", LogLevel::Error)
        .init();
    let dbg = Dbg::own("main");
    info!("starting up");
    let conf = "./config.yaml";
    let conf = Conf::new(&dbg, conf);
    let mut report = Report::new(
        &dbg,
        conf.api.params.ship_id.clone(),
        conf.api.params.project_id.clone(),
        "ru",
        ApiClient::new(
            &dbg,
            conf.api.address.database.clone(),
            conf.api.address.host.clone(),
            conf.api.address.port.clone(),
        ),
    );
    if let Err(error) = report.get_target(&conf.data.dir, &conf.data.name) {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
    if let Err(error) = report.get_ship_wide() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
    if let Err(error) = report.get_from_db() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
    if let Err(error) = report.write(&conf.data.dir, &conf.data.name) {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
    let mut report = Report::new(
        &dbg,
        conf.api.params.ship_id.clone(),
        conf.api.params.project_id.clone(),
        "en",
        ApiClient::new(
            &dbg,
            conf.api.address.database.clone(),
            conf.api.address.host.clone(),
            conf.api.address.port.clone(),
        ),
    );
    if let Err(error) = report.get_target(&conf.data.dir, &conf.data.name) {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
    if let Err(error) = report.get_ship_wide() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
    if let Err(error) = report.get_from_db() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
    if let Err(error) = report.write(&conf.data.dir, &conf.data.name) {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
}
