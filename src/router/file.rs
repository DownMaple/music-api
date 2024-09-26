
use alloc::vec::Vec;
use rocket::{routes, Route};
use crate::models::file::file_controller::download_file;

pub fn create_routes() -> Vec<Route> {
    routes![
        download_file,
    ]
}
