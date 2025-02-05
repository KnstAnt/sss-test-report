use db::api_server::ApiServer;
use log::info;
use parser::Report;
use std::io;
use std::io::*;

mod content;
mod db;
mod error;
mod formatter;
mod parser;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");
    let ship_id = 2;
    let path = "src/bin/SSS_Sofia_test1.xlsx";
    let language = Some("ru".to_owned());
    let mut report = Report::new(
        language.clone(),
        ship_id,
        ApiServer::new(
            "sss-computing".to_owned(),
            2,
            None,
            language,
        )
    );
    if let Err(error) = report.get_target(path) {
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
    if let Err(error) = report.write("src/bin/result_ru.md") {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
    let language = Some("en".to_owned());
    let mut report = Report::new(
        language.clone(),
        ship_id,
        ApiServer::new(
            "sss-computing".to_owned(),
            2,
            None,
            language,
        )
    );
    if let Err(error) = report.get_target(path) {
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
    if let Err(error) = report.write("src/bin/result_en.md") {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
}
