use std::path::PathBuf;

use rocket::{State, serde::json::Json};

use crate::Root;


#[rocket::get("/dir/<path..>")]
pub async fn dir(path: PathBuf, root: &State<Root>) -> Json<Vec<String>> {
    let root_path = root.0.clone();
    let mut path = root_path.join(path);

    if path.is_file() {
        path.pop();
    }

    let mut files = Vec::new();
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        files.push(name.to_string());
    }

    Json(files)
}