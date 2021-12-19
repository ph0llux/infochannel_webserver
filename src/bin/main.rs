// - extern crates
#[macro_use] extern crate rocket;

// - external
use rocket::fs::{FileServer, relative};

// - internal
use lib::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", FileServer::from(relative!("dashboard/build/web")))
    .mount("/api", routes![datecontrol, add_newsfeed, get_all_newsfeeds])
    .attach(NewsDatabase::fairing())
}