#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod db;
mod responder_wrappers;
mod routes;

use db::redis::pool;
use responder_wrappers::cors::CORS;
use rocket::response::NamedFile;
use std::env;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("ui/todo-rust/dist/index.html")
}

#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    let result = NamedFile::open(Path::new("ui/todo-rust/dist/").join(file));

    // always return 200
    if result.is_ok() {
        result.ok()
    } else {
        index().ok()
    }
}

fn main() {
    // let prod: String = "production".to_string();
    let routes = match env::var("ROCKET_ENV").as_ref().map(String::as_ref) {
        Ok("production") => routes![index, files],
        Ok("staging") => routes![index, files],
        Ok("development") => routes::dev::get_routes(),
        Ok(_) => routes::dev::get_routes(),
        Err(_e) => routes::dev::get_routes(),
    };
    rocket::ignite().mount("/", routes).manage(pool()).launch();
}
