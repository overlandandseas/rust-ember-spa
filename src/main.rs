#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("ui/todo-rust/dist/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let result = NamedFile::open(Path::new("ui/todo-rust/dist/").join(file));

    // always return 200
    if result.is_ok() {
        result.ok()
    } else {
        index().ok()
    }
}

#[error(404)]
fn match_200s() -> io::Result<NamedFile> {
    NamedFile::open("ui/todo-rust/dist/index.html")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .catch(errors![match_200s])
        .launch();
}
