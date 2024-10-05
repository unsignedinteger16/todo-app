mod website;
mod errors;

use rocket::{catchers, launch, routes};
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

use website::{ index };
use errors::{ error_catcher };

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/public", FileServer::from(relative!("/public")))
        .mount("/", routes![
            index,
        ])
        .register("/", catchers![error_catcher])
}