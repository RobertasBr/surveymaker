use rocket::routes;
use rocket::response::content;
use prism::render;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![hello])
    .launch()
}

#[get("/hello")]
fn hello() -> content::Html<&'static str> {
content::Html(prism::render("index.html", &()).unwrap())
}
