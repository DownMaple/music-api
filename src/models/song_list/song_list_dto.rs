use rocket::serde::{Deserialize, Serialize};
use sea_orm::prelude::DateTime;
use crate::models::music::music::music::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct SongListDto {
    pub id: i32,
    pub img: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub classify_id: Option<String>,
    pub classify_title: Option<String>,
    pub status: Option<i32>,
    pub create_time: Option<DateTime>,
    pub create_id: Option<i32>,
    pub update_time: Option<DateTime>,
    pub update_id: Option<i32>,
    pub music_list: Vec<Model>,
}