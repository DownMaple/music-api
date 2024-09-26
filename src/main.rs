extern crate alloc;

mod router;
mod utils;
mod models;

use rocket::{catchers};
use rocket::fs::{FileServer, relative};
use router::music;
use crate::router::{file, song_list};
use crate::utils::catcher::{not_found, unprocessable_entity};
use crate::utils::db::connect_to_db;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", file::create_routes())
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/music", music::create_routes())
        .mount("/song_list", song_list::create_routes())
        .register("/", catchers![not_found, unprocessable_entity])
        .manage(connect_to_db().await)
        .launch()
        .await?;

    Ok(())
}
