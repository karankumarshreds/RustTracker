use std::fs::File;

const FILE_NAME: &str = "db.json";

#[allow(dead_code)]
enum Commands {
    Get(String),
    Add(String),
    Delete(String),
    Update(String),
}

fn main() {
    let file_content = std::fs::read_to_string(FILE_NAME);
    if let Ok(fc) = file_content {
        return println!("{}", fc);
    }
    if let Err(err) = File::create(FILE_NAME) {
        return println!("{}", err.to_string());
    } 
    println!("File created: {}", FILE_NAME);
}




