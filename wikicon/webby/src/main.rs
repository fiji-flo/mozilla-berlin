#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::response::content::Html;
use rocket::response::status::Custom;

fn read_file(path: &str) -> Result<String, String> {
    let file = File::open(path).map_err(|e| {
        format!("unable to open {}: {}", path, e)
    })?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).map_err(|e| {
        format!("unable to read file {}: {}", path,e )
    })?;
    Ok(contents)
}

#[get("/render")]
fn render() -> Result<Html<String>, Custom<String>> {
    let template = read_file("slides.html").map_err(|e| {
        Custom(Status::InternalServerError, e)
    })?;
    let content = read_file("slides.md").map_err(|e| {
        Custom(Status::InternalServerError, e)
    })?;
    let output = template.replace("###SLIDES###", &content);
    Ok(Html(output))
}

#[get("/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(file).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![render, file]).launch();
}
