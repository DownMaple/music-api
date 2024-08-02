use rocket::serde::{Deserialize, Serialize};
use sea_orm::prelude::DateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMusicDto {
    pub img: String,
    pub title: String,
    pub lyrics: Option<String>,
    pub author_id: Option<i32>,
    pub author_name: Option<String>,
    pub album_id: Option<i32>,
    pub album_title: Option<String>,
    pub issued_time: DateTime,
    pub duration: Option<String>,
    pub link: String,
    pub classify_id: Option<String>,
    pub classify_title: Option<String>,
    pub views: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMusicDto {
    pub id: i32,
    pub img: String,
    pub title: String,
    pub lyrics: Option<String>,
    pub author_id: Option<i32>,
    pub author_name: Option<String>,
    pub album_id: Option<i32>,
    pub album_title: Option<String>,
    pub issued_time: DateTime,
    pub duration: Option<String>,
    pub link: String,
    pub classify_id: Option<String>,
    pub classify_title: Option<String>,
    pub views: Option<i32>,
    pub status: Option<i32>,
    pub create_time: Option<DateTime>,
    pub create_id: Option<i32>,
    pub update_time: Option<DateTime>,
    pub update_id: Option<i32>,
}

