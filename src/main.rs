#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use std::{path::PathBuf, str::FromStr};

use rocket::routes;
use rocket_contrib::serve::StaticFiles;

const UNKNOWN_PAGE: &str = "404";

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("./public"))
        .launch();
}
