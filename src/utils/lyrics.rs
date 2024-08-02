use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn get_lyrics(path: &str) -> String {
    let mut lyrics = String::new();
    let path = Path::new(path);
    let files = File::open(&path);
    match files {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line.unwrap();
                lyrics.push_str(&line);  // 将读取的行追加到歌词字符串
                lyrics.push(';');        // 追加分号
            }
        }
        Err(error) => lyrics = error.to_string()
    }
    lyrics
}