#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use std::{path::PathBuf, str::FromStr};

use auth::ApiKey;
use db::*;
use diesel::result::Error;
use model::{NewProject, Project};
use rocket::http::Status;
use rocket::{catch, catchers, response::content::Html};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

mod auth;
mod db;
mod model;
mod schema;

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount(
            "/api/projects",
            routes![get_proj, new_proj, edit_proj, edit_title, drop_proj],
        )
        .mount("/", StaticFiles::from("./public"))
        .register(catchers![catcher])
        .launch();
}

#[catch(404)]
fn catcher() -> Html<String> {
    let path = PathBuf::from_str("./public/index.html").unwrap();
    Html(std::fs::read_to_string(path).unwrap())
}

#[get("/<title>")]
fn get_proj(title: String, conn: DbConn) -> Result<Json<Project>, Status> {
    match get_project(&title, conn) {
        Ok(p) => Ok(Json(p)),
        Err(e) => Err(error_condition(e)),
    }
}

#[post("/new", format = "application/json", data = "<project>")]
fn new_proj(project: Json<Project>, _key: ApiKey, conn: DbConn) -> Status {
    match add_project(project.into_inner().into(), conn) {
        Ok(_) => Status::Accepted,
        Err(e) => error_condition(e),
    }
}

#[patch("/edit/<title>", format = "application/json", data = "<changes>")]
fn edit_proj(title: String, changes: Json<NewProject>, _key: ApiKey, conn: DbConn) -> Status {
    match update_project(title, changes.into_inner(), conn) {
        Ok(_) => Status::Accepted,
        Err(e) => error_condition(e),
    }
}

#[patch("/edit/<title>?<new>")]
fn edit_title(title: String, new: String, _key: ApiKey, conn: DbConn) -> Status {
    match update_title(title, new, conn) {
        Ok(_) => Status::Accepted,
        Err(e) => error_condition(e),
    }
}

#[delete("/delete/<title>")]
fn drop_proj(title: String, _key: ApiKey, conn: DbConn) -> Status {
    match delete_proj(title, conn) {
        Ok(_) => Status::Accepted,
        Err(e) => error_condition(e),
    }
}

fn error_condition(e: Error) -> Status {
    match e {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
