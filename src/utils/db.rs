use sea_orm::{Database, DatabaseConnection};
pub async fn connect_to_db() -> DatabaseConnection {
    let db = Database::connect("mysql://root:root@localhost/project_music").await.unwrap();
    db
}
