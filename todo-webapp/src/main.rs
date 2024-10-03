mod website;
mod errors;

use rocket::{launch, routes};
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

use website::{ index };

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/public", FileServer::from(relative!("/public")))
        .mount("/", routes![
            index,
        ])
}