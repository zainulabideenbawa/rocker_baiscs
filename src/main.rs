#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use rocket::http::RawStr;
use rocket::request::{Form, FormDataError, FormError};
use rocket::response::NamedFile;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, FromForm)]
pub struct Todo {
    pub description: String,
}
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<usize, Form<Todo>>> = Mutex::new({
        let mut m = HashMap::new();
        m
    });
}
static mut key: usize = 0;

#[post("/", data = "<todo>")]
fn testPost(todo: Form<Todo>) -> Option<NamedFile> {
    let mut map = HASHMAP.lock().unwrap();
    unsafe {
        key += 1;
        map.insert(key, todo);
        println!("{:?}", map);
        NamedFile::open("static/index.html").ok()
    }
}

#[delete("/<id>")]
fn testDelete(id: usize) -> Option<NamedFile> {
    let mut map = HASHMAP.lock().unwrap();
    unsafe {
       
        map.remove(&id);
        println!("{:?}", map);
        NamedFile::open("static/index.html").ok()
    }
}

// #[put("/<id>/<value>")]
// fn testPut(id: usize, value: String) -> Option<NamedFile> {
//     let mut map = HASHMAP.lock().unwrap();
//     unsafe {
       
//         map.insert(&id, value);
//         println!("{:?}", map);
//         NamedFile::open("static/index.html").ok()
//     }
// }
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
