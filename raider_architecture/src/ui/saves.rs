use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// Backup file path
const SAVE_FILE: &str = "src/assets/Saves/";

// Function to read saved name from file
pub fn save_name(filename: &str, line: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(format!("{}{}", SAVE_FILE, filename))
        .expect("Unable to open file");
    file.write_all(line.as_bytes())
        .expect("Unable to write to file");
}

pub fn delete_file(filepath: &str) {
    if fs::metadata(filepath).is_ok() {
        fs::remove_file(filepath).unwrap();
    }
}
