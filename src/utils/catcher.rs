use alloc::format;
use rocket::{catch, Request};
use rocket::serde::json::Json;
use crate::utils::res::{Res, res_error};

#[catch(404)]
pub fn not_found(req: &Request) -> Json<Res<()>> {
    res_error(404, &format!("Resource '{}' not found", req.uri()))
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> Json<Res<()>> {
    res_error(422, &format!("Invalid query parameter: {:?}", req.uri()))
}