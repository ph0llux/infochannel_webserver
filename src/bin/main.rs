// - extern crates
#[macro_use] extern crate rocket;

// - external
use rocket::fs::{FileServer};
#[cfg(debug_assertions)]
use rocket::fs::{relative};

// - internal
use lib::*;

#[launch]
fn rocket() -> _ {

    #[cfg(debug_assertions)]
    let fileserver_path = relative!("dashboard/build/web");
    #[cfg(not(debug_assertions))]
    let fileserver_path = "/var/www/dashboard";


    rocket::build()
    .mount("/", FileServer::from(fileserver_path))
    .mount("/api", routes![datecontrol, add_newsfeed, get_all_newsfeeds])
    .attach(NewsDatabase::fairing())
}