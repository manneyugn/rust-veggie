use rocket::fs::{FileServer, relative};

use rocket_dyn_templates::{Template, context};

#[rocket::get("/")]
pub fn index() -> Template  {
    Template::render("index", context! {})
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/static",  FileServer::from(relative!("/static")))
    .mount("/", rocket::routes![index])
    .attach(Template::fairing())
}