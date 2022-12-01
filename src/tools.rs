use std::fs;

pub fn read_file(location: &str) -> String {
    
    let file_content: String = fs::read_to_string(location)
        .expect("Could not read the file")
        .lines()
        .map(|chars| chars.to_string())
        .collect();
    
    file_content
}