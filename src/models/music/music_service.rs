use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder};
use crate::models::music::music::music;
use crate::models::music::music::music::{ActiveModel, Model as MusicModel, Model};
use crate::models::music::music::prelude::Music;
use sea_orm::ColumnTrait;
pub async fn get_music_by_id(db: &DatabaseConnection, id: i32) -> Option<MusicModel> {
    Music::find_by_id(id).one(db).await.ok().flatten()
}

pub async fn save_music(db: &DatabaseConnection, music: ActiveModel) -> Result<(),Box<dyn std::error::Error>> {
    music.insert(db).await?;
    Ok(())
}

pub async fn update_music(db: &DatabaseConnection, music: MusicModel) -> Result<MusicModel, DbErr> {
    let music_model = music.into_active_model().reset_all();
    let pear: MusicModel = music_model.update(db).await?;
    Ok(pear)
}

pub async fn get_music_list(db: &DatabaseConnection, page: i32, size: i32) -> Result<Vec<Model>, DbErr> {
    let list = Music::find().filter(music::Column::Status.eq(1)).order_by_desc(music::Column::CreateTime).paginate(db, size as u64).fetch_page(page as u64 - 1).await?;
    Ok(list)
}
