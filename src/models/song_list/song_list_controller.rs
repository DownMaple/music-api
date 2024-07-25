use rocket::{get, State};
use sea_orm::DatabaseConnection;
use crate::models::song_list::song_list::song_list::Model;
use crate::models::song_list::song_list_dto::SongListDto;
use crate::models::song_list::song_list_service;
use crate::utils::res::{res_success, ResType};

#[get("/getOne?<id>")]
pub async fn get_one(id: i32, db: &State<DatabaseConnection>) -> ResType<Option<SongListDto>> {
    let song_list: Option<SongListDto> = song_list_service::get_song_list_by_id(&db, id).await;
    Ok(res_success(song_list))
}

#[get("/list?<page>&<size>")]
pub async fn list(page:i32, size:i32, db: &State<DatabaseConnection>) -> ResType<Vec<Model>> {
    let song_list: Vec<Model> = song_list_service::get_song_list_list(&db, page, size).await.unwrap();
    Ok(res_success(song_list))
}
