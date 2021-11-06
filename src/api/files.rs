use std::{fs::read_to_string, io, path::PathBuf};

use comrak::{ComrakOptions, markdown_to_html};
use rocket::{State, response::content};

use crate::Root;


#[rocket::get("/files/raw/<path..>")]
pub async fn raw(path: PathBuf, root: &State<Root>) -> content::Plain<String> {
    let root_path = root.0.clone();
    let path = root_path.join(path);

    let markdown = fetch_markdown(path).unwrap();

    content::Plain(markdown)
}


#[rocket::get("/files/html/<path..>")]
pub async fn html(path: PathBuf, root: &State<Root>) -> content::Html<String> {
    let root_path = root.0.clone();
    let path = root_path.join(path);

    let markdown = fetch_markdown(path).unwrap();

    content::Html(markdown_to_html(&markdown[..], &ComrakOptions::default()))
}

fn fetch_markdown(path: PathBuf) -> Result<String, io::Error> {
    let mut path = path;
    if path.is_dir() {
        path.push("README.md");
    }

    read_to_string(path)
}

