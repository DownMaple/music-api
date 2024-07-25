
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Res<T> {
    code: u16,
    message: String,
    data: Option<T>,
}

impl<T> Res<T> {
    fn success(data: T) -> Self {
        Res {
            code: 200,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    fn error(code: u16, message: &str) -> Self {
        Res {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}

pub type ResType<T> = Result<Json<Res<T>>,Json<Res<()>>>;
pub fn res_success<T: Serialize>(data: T) -> Json<Res<T>> {
    Json(Res::success(data))
}
pub fn res_error(code: u16, message: &str) -> Json<Res<()>> {
    Json(Res::error(code, message))
}
