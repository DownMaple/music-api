use sea_orm::{ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Statement};
use crate::models::music::music::prelude::Music;
use crate::models::song_list::song_list::prelude::SongList;
use crate::models::song_list::song_list::song_list;
use crate::models::song_list::song_list::song_list::Model;
use crate::models::song_list::song_list_dto::SongListDto;

pub async fn get_song_list_by_id(db: &DatabaseConnection, id: i32) -> Option<SongListDto> {
    let song: Option<Model> = SongList::find_by_id(id).one(db).await.ok().flatten();
    if let Some(song) = song {
        let sql = "SELECT
            music.*,
            music.id as music_id,
            music_song_list.song_list_id as song_list_id
        from music
        JOIN music_song_list ON music.id = music_song_list.music_id
        where song_list_id = ?";
        let music_list = Music::find().from_raw_sql(
            Statement::from_sql_and_values(
                DbBackend::MySql,
                sql,
                vec![id.into()],
            )
        ).all(db).await.unwrap();
        let song_msg = SongListDto {
            id: song.id,
            img: song.img,
            title: song.title,
            author: song.author,
            description: song.description,
            classify_id: song.classify_id,
            classify_title: song.classify_title,
            status: song.status,
            create_time: song.create_time,
            create_id: song.create_id,
            update_time: song.update_time,
            update_id: song.update_id,
            music_list,
        };
        Some(song_msg)
    } else {
        None
    }
}

pub async fn get_song_list_list(db: &DatabaseConnection, page: i32, size: i32) -> Result<Vec<Model>, DbErr> {
    let list = SongList::find().filter(song_list::Column::Status.eq(1)).order_by_desc(song_list::Column::CreateTime).paginate(db, size as u64).fetch_page(page as u64 - 1).await?;
    Ok(list)
}