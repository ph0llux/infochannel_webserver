// - extern crates
#[macro_use] extern crate rocket;

// - external
use rocket::fs::{FileServer, relative};

// - modules
mod web_api;
mod constants;

// - re-exports
pub use web_api::*;
pub use constants::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", FileServer::from(relative!("dashboard/build/web")))
    .mount("/api", routes![datecontrol])
}