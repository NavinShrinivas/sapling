#[allow(dead_code)]
#[allow(non_snake_case)]
use rocket;
// use std::rc::Rc;
// use crate::RenderEnv;

#[rocket::launch]
pub fn rocket_serve() -> _ {
    rocket::build().mount(
        "/",
        rocket::fs::FileServer::from(rocket::fs::relative!("static")),
    )
}
