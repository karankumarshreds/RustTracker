use std::fs::{File, read_to_string};
use serde::{Deserialize};

const FILE_NAME: &str = "db.json";

#[allow(dead_code)]
enum Commands {
    Get(String),
    Add(String),
    Delete(String),
    Update(String),
}

#[derive(Debug)]
enum Status {
    Pending(String),
    Completed(String),
    Unknown(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="lowercase")]
#[allow(dead_code)]
struct Item {
    status: Status,
    message: String,
    id: usize,
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
        D: serde::Deserializer<'de> {
            let s = String::deserialize(deserializer)?;
            if s == "pending" {
                return Ok(Status::Pending(String::from("pending")));
            } else if s == "completed" {
                return Ok(Status::Completed(String::from("completed")));
            } else {
                return Ok(Status::Unknown(String::from("unknown")));
            }
    }
}

fn main() {
    let file_content = read_to_string(FILE_NAME);

    if let Ok(file_content) = file_content {
        match serde_json::from_str::<Vec<Item>>(&file_content) {
            Ok(items) => println!("{:?}", items),
            Err(err) => println!("ERROR: {}", err.to_string()),
        }
        return;
    } else {
        println!("Oops...");
    }

    if let Err(err) = File::create(FILE_NAME) {
        return println!("{}", err.to_string());
    } 
    println!("File created: {}", FILE_NAME);
}





