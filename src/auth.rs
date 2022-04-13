use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

pub struct ApiKey(String);

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut keys = include_str!("keys").rsplit(['\n']);
        let headers = request.headers();
        if let Some(k) = headers.get_one("Key") {
            if keys.any(|v| k == v) {
                Outcome::Success(ApiKey(k.to_string()))
            } else {
                Outcome::Failure((Status::Forbidden, ()))
            }
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}
