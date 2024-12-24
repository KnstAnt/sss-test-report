use db::api_server::ApiServer;
use log::info;
use parser::Report;
use std::cell::RefCell;
use std::io;
use std::io::*;
use std::rc::Rc;

mod content;
mod db;
mod error;
mod formatter;
mod parser;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");
    let mut report = Report::new(2, ApiServer::new("sss-computing".to_owned()));
    if let Err(error) = report.get_target("src/bin/SSS_Sofia_test3.xlsx") {
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
    if let Err(error) = report.write("src/bin/result.md") {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        //       println!("{}", error.to_string());
        return;
    }
}
