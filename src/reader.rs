use std::{fs::{self, read_dir, remove_file}, io::Cursor, path::Path};

use audiotags::Tag;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, to_writer_pretty};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Metadata {
    index: u32,
    source: String,
    title: String,
    artist: String,
    picture: String,
}

pub fn read_assets() -> Vec<Metadata> {
    let _ = fs::remove_dir_all("./output");
    fs::create_dir("./output").unwrap();
    fs::create_dir("./output/images").unwrap();
    let mut result: Vec<Metadata> = Vec::new();
    let dir = read_dir("./assets").unwrap();
    let mut index = 1;
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("Processing: {:?}", &path);
        if path.is_file() {
            let tag = Tag::new().read_from_path(&path);
            if let Ok(info) = tag {
                let mut meta = Metadata {
                    index,
                    source: path.to_str().unwrap().to_string(),
                    title: "Unknown".to_string(),
                    artist: "Unknown".to_string(),
                    picture: "Unknown".to_string(),
                };
                if let Some(title) = info.title() {
                    meta.title = title.to_string();
                }
                if let Some(artist) = info.artist() {
                    meta.artist = artist.to_string();
                }
                if let Some(picture) = info.album_cover() {
                    let img = ImageReader::new(Cursor::new(picture.data)).with_guessed_format().unwrap().decode().unwrap();
                    img.save(Path::new(format!("./output/images/{}.png", index).as_str())).unwrap();
                    meta.picture = format!("./output/images/{}.png", index);
                }
                index += 1;
                result.push(meta);
            }
        }
    }
    return result
}

pub fn write_to_json(result: Vec<Metadata>) {
    let _ = remove_file("./output/music_metadata.json");
    let js_string: Vec<String> = result.iter().map(|m| { to_string_pretty(m).unwrap() }).collect();
    let filename = fs::File::create("./output/music_metadata.json").unwrap();
    to_writer_pretty(filename, &js_string).unwrap();
}
