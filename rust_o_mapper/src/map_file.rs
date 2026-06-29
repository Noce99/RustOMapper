//! This file contains:
//! 1) Definition and implementation of a MapFile struct.
//! 2) An helper function to get all the maps in the hardcoded "Maps" folder.
//! It publish 2 modules:
//! 1) reading -> Used to parse a .omap file to a RAM reppresentation
//! 2) writing -> Used to write a .omap file from a RAM reppresentation [!TODO]

use crate::map::Map;

pub mod reading;
pub mod writing;

/// This struct represent a .omap file.
/// It can be in two differents states:
/// 1) Non yet parsed map file (map is None)
/// 2) Parsed map file (map contains the map RAM reppresentation)
pub struct MapFile {
    /// The absolute path to a .omap file
    pub path: String,
    /// If the map was already parsed it contains the map RAM reppresentation otherwise None
    pub map: Option<Map>,
}

impl MapFile {
    /// Given a path it creates a NON parsed MapFile instance
    pub fn new(path: String) -> MapFile {
        MapFile { path, map: None }
    }
    /// Given a path it created a parsed MapFile instance
    pub fn new_and_load(path: String) -> MapFile {
        let map = Map::new(&path, true);
        MapFile { path, map}
    }
    /// In read data from the disk and create a RAM reppresentation of the map
    /// If the map was already parsed, it will do nothing.
    pub fn load(&mut self) {
        if self.map.is_none() {
            self.map = Map::new(self.path.as_str(), true)
        }
    }
    /// In read data from the disk and create a RAM reppresentation of the map
    /// If the map was already parsed, it parses it again.
    pub fn load_force(&mut self) {
        self.map = Map::new(self.path.as_str(), true)
    }
}


pub mod map_finder {
    use crate::map_file::MapFile;
    use std::{env, fs, path::PathBuf};

    /// This function return a vector of MapFile istances for all the .omap file
    /// contained in the Maps folder (sibling of src folder).
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

    fn search_for_omap_in_subtree(root : &PathBuf) -> Vec<PathBuf>{
        let mut maps_path: Vec<PathBuf> = Vec::new();

        let root_path = match &root.to_str(){
            Some(a_path) => a_path.to_string(),
            None => {
                eprintln!("Path it's not UTF-8 valid.");
                return maps_path;
            }
        };
        match fs::read_dir(root) {
            Ok(entries) => {
                for result_entry in entries{
                    match result_entry{
                        Ok(entry) => {
                            let entry_path = match entry.path().to_str(){
                                Some(a_path) => a_path.to_string(),
                                None => {
                                    eprintln!("Path it's not UTF-8 valid.");
                                    continue;
                                }
                            };
                            let file_type = match entry.file_type(){
                                Ok(a_file_type) => a_file_type,
                                Err(error) => {
                                    eprintln!("Skipping a child of {} because of:\n{}", root_path, error);
                                    continue;
                                },
                            };
                            if file_type.is_dir(){
                                maps_path.extend(search_for_omap_in_subtree(&entry.path()));
                            }else if file_type.is_file() || file_type.is_symlink(){
                                if entry_path.ends_with(".omap"){
                                    maps_path.push(entry.path());
                                }
                            }else{
                                eprintln!("The following entry was either a folder, a file or a symbolic link.\n{}", entry_path);
                                continue;
                            }
                        },
                        Err(error) => {
                            eprintln!("Skipping a child of {} because of:\n{}", root_path, error);
                        }
                    }
                }
            },
            Err(error) => {
                eprintln!("Skipping to search on folder: {} because of:\n{}", root_path, error);
               return maps_path;
            }
        }
        maps_path
    }

    // TESTS
    #[cfg(test)]
    mod tests {
        use std::{env, fs};

        use super::*;

        fn create_a_random_name() -> String{
            let mut name = String::new();
            for _ in 0..rand::random_range(1..=10){
                name.push(rand::random_range('a'..='z'));
            }
            // TODO: add also non UTF-8 name creation
            name
        }

        fn create_random_subtree(root : &PathBuf, probability_to_stop : f32) -> Vec<PathBuf>{
            let mut maps_path : Vec::<PathBuf> = Vec::new();
            assert!(0. <= probability_to_stop && probability_to_stop <= 1.);
            if rand::random_range(0.0..=1.0) > probability_to_stop {
                let number_of_subfolders = rand::random_range(1..=10);
                let number_of_non_map_files = rand::random_range(1..=10);
                let number_of_map_files = rand::random_range(1..=10);

                for _ in 0..number_of_subfolders{
                    let sub_folder_path = root.join(create_a_random_name());
                    match fs::create_dir(&sub_folder_path){
                        Ok(_) => maps_path.extend(
                            create_random_subtree(&sub_folder_path, probability_to_stop+(1.-probability_to_stop)*0.5)
                        ),
                        Err(error) => eprintln!("Error creating directory {}:\n{}", sub_folder_path.to_str().unwrap_or("non_utf_8_path"), error)
                    }
                }
                for _ in 0..number_of_non_map_files{
                    let file_path = root.join(create_a_random_name());
                    match fs::write(&file_path, "") {
                        Ok(_) => {},
                        Err(error) => eprintln!("Error creating file {}:\n{}", file_path.to_str().unwrap_or("non_utf_8_path"), error)
                    };
                }
                for _ in 0..number_of_map_files{
                    let omap_file_path = root.join(create_a_random_name() + ".omap");                    
                    match fs::write(&omap_file_path, ""){
                        Ok(_) => maps_path.push(omap_file_path),
                        Err(error) => eprintln!("Error creating file {}:\n{}", omap_file_path.to_str().unwrap_or("non_utf_8_path"), error)
                    }; 
                }
            }
            maps_path
        }

        #[test]
        fn test_search_for_omap_in_subtree() {
            let tmp_dir_path = env::temp_dir().join(create_a_random_name());
            fs::create_dir(&tmp_dir_path).expect("Not possible to create tmp dir, aborting the test!");
            let mut gt_maps_path = create_random_subtree(&tmp_dir_path, 0.0);
            let mut maps_path = search_for_omap_in_subtree(&tmp_dir_path);
            if fs::remove_dir_all(tmp_dir_path).is_err(){
                println!("Failed to delete the tmp folder that I created! :-(");
            }
            gt_maps_path.sort();
            maps_path.sort();
            println!("gt_maps_path:");
            for element in &gt_maps_path{
                println!("\t{}", element.to_str().unwrap());
            }
            println!("maps_path:");
            for element in &maps_path{
                println!("\t{}", element.to_str().unwrap());
            }
            assert_eq!(gt_maps_path, maps_path);
        }
    }
}
