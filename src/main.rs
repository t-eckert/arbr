extern crate rocket;

use comrak::{markdown_to_html, ComrakOptions};
use rocket::fs::NamedFile;
use rocket::response::content;
use std::path::PathBuf;
use std::fs::read_to_string;


#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/api", rocket::routes![serve_raw, serve_html])
        .launch()
        .await
        .expect("Failed to launch rocket");
}

#[rocket::get("/raw/<path..>")]
async fn serve_raw(path: PathBuf) -> Option<NamedFile> {

    let root = PathBuf::from("/home/thomaseckert/Notebook/");
    let mut path = root.join(path);
    if path.is_dir() {
        path.push("README.md");
    }

    NamedFile::open(path).await.ok()
}


#[rocket::get("/html/<path..>")]
async fn serve_html(path: PathBuf) -> Option<content::Html<String>> {

    let root = PathBuf::from("/home/thomaseckert/Notebook/");
    let mut path = root.join(path);
    if path.is_dir() {
        path.push("README.md");
    }

    Some(content::Html(markdown_to_html(read_to_string(path).as_deref().unwrap_or(""), &ComrakOptions::default())))
}