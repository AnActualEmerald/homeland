#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use std::{path::PathBuf, str::FromStr};

use rocket::{catch, catchers, response::content::Html};
use rocket_contrib::serve::StaticFiles;

const UNKNOWN_PAGE: &str = "404";

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("./public"))
        .register(catchers![catcher])
        .launch();
}

#[catch(404)]
fn catcher() -> Html<String> {
    let path = PathBuf::from_str("./public/index.html").unwrap();
    Html(std::fs::read_to_string(path).unwrap())
}
