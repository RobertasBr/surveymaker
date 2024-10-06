#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> &'static str {
    include_str!("../static/index.html")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
}
