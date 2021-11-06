extern crate rocket;

use clap::{crate_version, crate_authors, Parser};
use comrak::{markdown_to_html, ComrakOptions};
use rocket::{State, response::content};
use std::{io, fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    #[clap(short, long, default_value = "./")]
    root: String,
}

struct Root(PathBuf);

#[rocket::main]
async fn main() {
    let opts = Opts::parse();
    let root = PathBuf::from(opts.root);

    rocket::build()
        .manage(Root(root))
        .mount("/api", rocket::routes![raw, html])
        .launch()
        .await
        .expect("Failed to launch rocket");
}

fn fetch_markdown(path: PathBuf) -> Result<String, io::Error> {
    let mut path = path;
    if path.is_dir() {
        path.push("README.md");
    }

    read_to_string(path)
}

#[rocket::get("/raw/<path..>")]
async fn raw(path: PathBuf, root: &State<Root>) -> Option<content::Plain<String>> {
    let root_path = root.0.clone();
    let path = root_path.join(path);

    let markdown = fetch_markdown(path).unwrap();

    Some(content::Plain(markdown))
}


#[rocket::get("/html/<path..>")]
async fn html(path: PathBuf, root: &State<Root>) -> Option<content::Html<String>> {
    let root_path = root.0.clone();
    let path = root_path.join(path);

    let markdown = fetch_markdown(path).unwrap();

    Some(content::Html(markdown_to_html(&markdown[..], &ComrakOptions::default())))
}