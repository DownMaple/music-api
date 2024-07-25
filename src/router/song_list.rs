
use alloc::vec::Vec;
use rocket::{routes, Route};
use crate::models::song_list::song_list_controller::{get_one, list};

pub fn create_routes() -> Vec<Route> {
    routes![
        get_one,
        list
    ]
}
