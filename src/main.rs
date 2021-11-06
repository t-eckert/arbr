extern crate rocket;

mod fairings;
mod api;

use api::{dir::dir, files::{raw, html}};
use clap::{crate_version, crate_authors, Parser};
use fairings::CORS;
use std::path::PathBuf;


#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    #[clap(short, long, default_value = "./")]
    root: String,
}

pub struct Root(PathBuf);

#[rocket::main]
async fn main() {
    let opts = Opts::parse();
    let root = PathBuf::from(opts.root);

    rocket::build()
        .manage(Root(root))
        .mount("/api", rocket::routes![raw, html, dir])
        .attach(CORS)
        .launch()
        .await
        .expect("Failed to launch rocket");
}
