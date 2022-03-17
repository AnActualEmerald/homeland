#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use std::{path::PathBuf, str::FromStr};

use rocket::routes;

fn main() {
    rocket::ignite().mount("/", routes![serve]).launch();
}

#[get("/<file>")]
fn serve(file: String) -> String {
    if let Ok(path) = PathBuf::from_str(&file) {
        if path.is_file() {
            let file = std::fs::read_to_string(path)
                .or::<String>(Ok(String::from("404")))
                .unwrap();
            file
        } else {
            format!("404")
        }
    } else {
        format!("404")
    }
}
