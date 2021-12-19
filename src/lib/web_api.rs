// - STD
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
// - STD
use std::{
    time::{SystemTime,UNIX_EPOCH},
};

// - external
use rocket::serde::{
    Serialize,
    Deserialize,
    json::Json,
};
use rocket::http::Status;

// - internal
use crate::constants::*;
use super::{NewsFeed,NewsDatabase};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct IncomingNewsfeed {
    headline: String,
    content: String,
    status: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatecontrolData {
    user: String,
    timestamp: i64
}

/// returns a json in following format:
/// ```
/// {
///     [
///         "user" : "bk123456xy",
///         "timestamp" : 1619698026
///     ]
/// }
/// ```
#[get("/datecontrol")]
pub async fn datecontrol() -> Json<Vec<DatecontrolData>> {
    let mut data = Vec::new();
    let entry_path = PathBuf::from(DATECONTROL_FILE_DIR);
    for entry in read_dir(entry_path).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            let path2 = path.clone();
            let user = path2.file_name().unwrap().to_string_lossy();
            let timestamp: i64 = read_to_string(path).unwrap().trim().parse().unwrap();
            let datecontrol = DatecontrolData {
                user: user.to_string(),
                timestamp: timestamp
            };
            data.push(datecontrol);
        }
    }
    Json::from(data)
}

/// expects a json in following format:
/// ```
/// {
///     "headline" : "Your awesome headline",
///     "content" : "your unbelievable content ;)",
///     "status" : 0,
/// }
/// ```
#[post("/add_newsfeed", format="json", data="<feed>")]
pub async fn add_newsfeed(conn: NewsDatabase, feed: Json<IncomingNewsfeed>) -> Status {
    let creation_date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let result = conn.run(move |c| NewsFeed::add_feed(c, creation_date as i64, &feed.headline, &feed.content, feed.status));
    match result.await {
        Ok(_) => Status::Created,
        Err(_) => Status::ImATeapot,
    }
}

/// returns a json in following format:
/// ```
/// {
///     [
///         "timestamp" : 1634729374,
///         "headline" : "Your awesome headline",
///         "content" : "your unbelievable content ;)",
///         "status" : 0,
///     ]
/// }
/// ```
#[get("/get_all_newsfeeds")]
pub async fn get_all_newsfeeds(conn: NewsDatabase) -> Json<Vec<NewsFeed>> {
    let result = conn.run(move |c| NewsFeed::get_all_newsfeeds(c));
    Json::from(result.await.unwrap())
}