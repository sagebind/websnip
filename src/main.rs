extern crate chrono;
extern crate hyper;
extern crate iron;
extern crate json;
#[macro_use] extern crate log;
extern crate logger;
extern crate simplelog;
extern crate wkhtmltopdf;

mod api;
mod mercury;
mod pdf;

use iron::prelude::*;
use logger::Logger;
use simplelog::{Config, LogLevelFilter, SimpleLogger};


fn main() {
    SimpleLogger::init(LogLevelFilter::Info, Config::default()).unwrap();

    let (logger_before, logger_after) = Logger::new(None);

    let mut handler = Chain::new(api::ApiHandler::new());
    handler.link_before(logger_before);
    handler.link_after(logger_after);

    Iron::new(handler).http(("0.0.0.0", 80)).unwrap();
}
