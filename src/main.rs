extern crate rocket;

use rocket::fs::NamedFile;
use std::path::PathBuf;


#[rocket::get("/<path..>")]
async fn serve(path: PathBuf) -> Option<NamedFile> {

    let root = PathBuf::from("/home/thomaseckert/Notebook/");
    let mut path = root.join(path);
    if path.is_dir() {
        path.push("README.md");
    }

    println!("{:?}", path);
    NamedFile::open(path).await.ok()
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![serve])
}
