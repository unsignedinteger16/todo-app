use rocket::{ get };
use rocket_dyn_templates::Template;

#[get("/")]
pub fn index() -> Template
{
    return Template::render("index", ());
}