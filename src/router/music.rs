
use alloc::vec::Vec;
use rocket::{routes, Route};
use crate::models::music::music_controller::{get_one, list, save, update};

pub fn create_routes() -> Vec<Route> {
    routes![
        get_one,
        list,
        save,
        update
    ]
}
