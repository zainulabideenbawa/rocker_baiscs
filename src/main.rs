#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket::request::{Form, FormDataError, FormError};
use rocket::response::NamedFile;


#[derive(Debug, FromForm)]
pub struct Todo {
    pub description: String,
}

#[post("/add", data = "<todo>")]
fn testPost(todo: Form<Todo>) {
    println!("{:?}", todo)
}
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, testPost])
}

fn main() {
    rocket().launch();
}
