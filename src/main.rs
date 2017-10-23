#[macro_use]
extern crate lazy_static;
extern crate futures;
#[macro_use]
extern crate hyper;
extern crate url;
extern crate mime;
extern crate handlebars;
extern crate crypto;
extern crate csrf;
extern crate data_encoding;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate walkdir;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate diesel;
extern crate regex;

mod config;
mod controller;
mod model;
mod view;

use config::Config;
use controller::router::RouterService;
use hyper::server::Http;

fn main()
{
    let addr = Config::idem().base_url.parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(RouterService)).unwrap();
    server.run().unwrap();
}