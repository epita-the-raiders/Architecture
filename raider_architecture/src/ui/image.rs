use std::{fs, path::Path};

pub fn load_images(path: String) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut file_names = Vec::new();

    for path in paths {
        let file_name = path.unwrap().path().display().to_string();
        file_names.push(file_name);
    }
    file_names.retain(|p| p.ends_with(".png"));
    file_names.sort();
    file_names.reverse();
    file_names
}

pub fn get_filename(path: &str) -> Option<String> {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

pub fn save_exists(save_number: &str) -> bool {
    let save_filename = format!("src/assets/Saves/Save{}.png", save_number);
    let images_to_load = load_images("src/assets/Saves/".to_string());
    images_to_load.contains(&save_filename)
}
