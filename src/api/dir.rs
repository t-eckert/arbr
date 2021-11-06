use std::path::PathBuf;

use rocket::{State, serde::{json::Json, Serialize}};

use crate::Root;


#[rocket::get("/dir/<path..>")]
pub async fn dir(path: PathBuf, root: &State<Root>) -> Json<Dir> {
    let root_path = root.0.clone();
    let mut path = root_path.join(path);

    if path.is_file() {
        path.pop();
    }

    let mut dir = Dir{
        files: Vec::new(),
        markdown: Vec::new(),
        directories: Vec::new(),
    };

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();

        if path.is_file() {
            dir.files.push(name.to_string());
            if name.ends_with(".md") {
                dir.markdown.push(name.to_string());
            }
        } else if path.is_dir() && ! name.starts_with(".") {
            dir.directories.push(name.to_string());
        };
    }

    Json(dir)
}

#[derive(Serialize)]
pub struct Dir {
    files: Vec<String>,
    markdown: Vec<String>,
    directories: Vec<String>,
}