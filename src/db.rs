use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use super::model::{NewProject, Project};
use super::schema;

use std::collections::HashMap;
use std::env;
use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_project(p_title: &str, conn: DbConn) -> Result<Project, Error> {
    use schema::projects::dsl::*;
    projects.filter(title.eq(p_title)).first(&*conn)
}

pub fn add_project(proj: NewProject, conn: DbConn) -> Result<Project, Error> {
    use schema::projects;

    diesel::insert_into(projects::table)
        .values(proj)
        .get_result(&*conn)
}

pub fn update_project(
    p_title: String,
    changes: NewProject,
    conn: DbConn,
) -> Result<Project, Error> {
    use schema::projects;
    use schema::projects::dsl::*;
    diesel::update(projects::table)
        .filter(title.eq(p_title))
        .set(changes)
        .get_result(&*conn)
}

pub fn update_title(p_title: String, n_title: String, conn: DbConn) -> Result<Project, Error> {
    use schema::projects;
    use schema::projects::dsl::*;
    diesel::update(projects::table)
        .filter(title.eq(p_title))
        .set(title.eq(n_title))
        .get_result(&*conn)
}

pub fn delete_proj(p_title: String, conn: DbConn) -> Result<usize, Error> {
    use schema::projects;
    use schema::projects::dsl::title;
    diesel::delete(projects::table)
        .filter(title.eq(p_title))
        .execute(&*conn)
}

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("couldn't make db pool")
}

pub fn database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").expect("unable to fine database_url in environment")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
