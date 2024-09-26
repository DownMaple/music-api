use std::io::SeekFrom;
use rocket::{get, response, Response};
use std::path::{Path, PathBuf};
use rocket::http::{ContentType, Status};
use rocket::response::{Responder};
use tokio::fs::File;
use rocket::request::{FromRequest, Request};
use rocket::outcome::Outcome;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

pub struct RangeHeader {
    start: Option<u64>,
    end: Option<u64>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RangeHeader {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        if let Some(range) = request.headers().get("Range").next() {
            println!("Range: {}", range);
            if let Some(range_str) = range.strip_prefix("bytes=") {
                let parts: Vec<&str> = range_str.split('-').collect();
                let start = parts.get(0).and_then(|s| s.parse::<u64>().ok());
                let end = parts.get(1).and_then(|s| s.parse::<u64>().ok());

                return Outcome::Success(RangeHeader { start, end });
            }
        }
        Outcome::Forward(Status::new(200))
    }
}

#[get("/static/music/<file_path..>")]
pub async fn download_file(file_path: PathBuf, range: RangeHeader) -> Result<FileResponse, Status> {

    // 读取文件
    let file_path = Path::new("static/music/").join(file_path);
    let mut file = File::open(&file_path).await.map_err(|_| Status::NotFound).expect("File not found");
    let metadata = file.metadata().await.map_err(|_| Status::InternalServerError).expect("Failed to get file metadata");
    let file_size = metadata.len();

    // 获取 Range 信息
    let (range_start, range_end) = match (range.start, range.end) {
        (Some(start), Some(end)) => (start, end),
        (Some(start), None) => (start, file_size - 1),
        (None, Some(end)) => (0, end),
        (None, None) => (0, file_size - 1),
    };

    // 读取文件的字节范围
    let mut buffer = vec![0; (range_end - range_start + 1) as usize];
    file.seek(SeekFrom::Start(range_start)).await.map_err(|_| Status::InternalServerError)?;
    file.read_exact(&mut buffer).await.map_err(|_| Status::InternalServerError)?;

    Ok(FileResponse {
        buffer,
        range_start,
        range_end,
        file_size,
    })
}

// 自定义响应类型，用于断点续传
pub struct FileResponse {
    buffer: Vec<u8>,
    range_start: u64,
    range_end: u64,
    file_size: u64,
}
impl<'r> Responder<'r, 'static> for FileResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {

        let string = format!("{}:{}:{}", self.range_start, self.range_end, self.file_size);
        let range_str = format!("bytes {}-{}/{}", self.range_start, self.range_end, self.file_size);
        Response::build_from(string.respond_to(req)?)
            .raw_header("Accept-Ranges", "bytes")
            .raw_header("Content-Range", range_str)
            .header(ContentType::new("application", "x-person"))
            .status(Status::Ok)
            .sized_body(self.buffer.len(), std::io::Cursor::new(self.buffer))
            .ok()
    }
}

