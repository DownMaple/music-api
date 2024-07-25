use rocket::{get, post, State};
use rocket::serde::json::Json;
use sea_orm::{DatabaseConnection, NotSet};
use sea_orm::ActiveValue::Set;
use crate::models::music::music_service;
use crate::utils::res::{res_error, res_success, ResType};
use crate::models::music::music::music::{ActiveModel, Model as MusicModel, Model};
use crate::models::music::music_dto::{CreateMusicDto};

#[get("/getOne?<id>")]
pub async fn get_one(id: i32, db: &State<DatabaseConnection>) -> ResType<Option<MusicModel>> {
    let music: Option<MusicModel> = music_service::get_music_by_id(&db, id).await;
    Ok(res_success(music))
}

#[get("/list?<page>&<size>")]
pub async fn list(page:i32, size:i32, db: &State<DatabaseConnection>) -> ResType<Vec<MusicModel>> {
    let music: Vec<MusicModel> = music_service::get_music_list(&db, page,size).await.unwrap();
    Ok(res_success(music))
}


#[post("/save", format = "json", data = "<music_data>")]
pub async fn save(music_data: Json<CreateMusicDto>, db: &State<DatabaseConnection>) -> ResType<()> {
    let music = music_data.into_inner();
    let music_model = ActiveModel {
        id: NotSet,
        img: Set(music.img),
        title: Set(music.title),
        author_id: Set(music.author_id),
        author_name: Set(music.author_name),
        album_id: Set(music.album_id),
        album_title: Set(music.album_title),
        issued_time: Set(music.issued_time),
        duration: Set(music.duration),
        link: Set(music.link),
        classify_id: Set(music.classify_id),
        classify_title: Default::default(),
        views: Default::default(),
        status: Default::default(),
        create_time: Default::default(),
        create_id: Default::default(),
        update_time: Default::default(),
        update_id: Default::default(),
    };
    match music_service::save_music(&db, music_model).await {
        Ok(_) => Ok(res_success(())),
        Err(e) => Ok(res_error(500, &e.to_string()))
    }
}

#[post("/update", format = "json", data = "<music_data>")]
pub async fn update(music_data: Json<Model>, db: &State<DatabaseConnection>) -> ResType<()> {
    let music = music_data.into_inner();
    match music_service::update_music(&db, music).await {
        Ok(_) => Ok(res_success(())),
        Err(e) => Ok(res_error(500, &e.to_string()))
    }
}