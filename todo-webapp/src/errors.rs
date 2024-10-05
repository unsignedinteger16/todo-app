use rocket::{catch, Request};
use rocket::form::Strict;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

#[catch(default)]
pub fn error_catcher(status: Status, req: &Request) -> Template {
    let reason: String = status.reason().unwrap_or(
        format!("Error {}", status.code).as_str(),
    ).to_string();

    Template::render("error", context! {
        err_reason: reason,
        uri: req.uri().to_string(),
    })
}