
pub mod prelude {
    pub use super::music::Entity as Music;
}

pub mod music {
    use chrono::{Local};
    use rocket::serde::{Deserialize, Serialize};
    use sea_orm::ActiveValue::Set;
    use sea_orm::entity::prelude::*;
    use sea_orm::prelude::async_trait::async_trait;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize )]
    #[sea_orm(table_name = "music")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)]
        pub id: i32,
        pub img: String,
        pub title: String,
        pub author_id: Option<i32>,
        pub author_name: Option<String>,
        pub album_id: Option<i32>,
        pub album_title: Option<String>,
        pub issued_time: DateTime,
        pub duration: Option<String>,
        pub link: String,
        pub classify_id: Option<String>,
        pub classify_title: Option<String>,
        #[sea_orm(default_value = "0")]
        pub views: Option<i32>,
        #[sea_orm(default_value = "1")]
        pub status: Option<i32>,
        pub create_time: Option<DateTime>,
        pub create_id: Option<i32>,
        pub update_time: Option<DateTime>,
        pub update_id: Option<i32>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}


    #[async_trait]
    impl ActiveModelBehavior for ActiveModel {

        /// Will be triggered before insert / update
        async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
        where
            C: ConnectionTrait,
        {
            // 获取当前日期时间，格式为 SeaORM 的 DateTime
            let now = Local::now().naive_local();
            if insert {
                self.create_time = Set(Option::from(now));
            }
            self.update_time = Set(Option::from(now));
            Ok(self)
        }
    }
}
