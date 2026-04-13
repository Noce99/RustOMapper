use crate::map::Map;
use std::{env, fs};

pub mod reading;
pub mod writing;
// pub mod explorer;

pub struct MapFile {
    pub path: String,
    pub map: Option<Map>,
}

impl MapFile {
    pub fn new(path: String) -> MapFile {
        MapFile { path, map: None }
    }
    pub fn load(&mut self) {
        self.map = Map::new(self.path.as_str(), true);
    }
}

pub fn get_map_paths() -> Vec<MapFile>{
    let mut maps_path: Vec<MapFile> = Vec::new();
    let mut current_dir = env::current_dir().expect("Failed to get current directory! WTF!");
    current_dir.push(r"Maps");
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(Result::ok) {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                let path_string = entry.path().to_str().unwrap().to_string();
                if path_string.ends_with(".omap") {
                    maps_path.push(MapFile::new(String::from(path_string)));
                }
            }
        }
    }
    maps_path
}