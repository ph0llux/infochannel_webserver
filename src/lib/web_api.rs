// - STD
use std::io::{BufRead};
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, File};
use std::path::{PathBuf, Path};
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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct WGClient {
    name: String,
    pub_key: String,
    creation_date: String,
    bytes_sent: String,
    bytes_received: String,
    latest_handshake: String,
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

/// returns a json in following format:
/// ```
/// {
///     [
///            "time_period" : "2022-02",
///            "public_ips" : {
///                 "2022-02-24-13:38:18" : "255.255.255.255"
///             }
///     ],   
/// }
#[get("/public_ip")]
pub async fn public_ip() -> Json<HashMap<String, HashMap<String, String>>> {
    let entry_path = PathBuf::from(PUBLIC_IP_PATH);
    let mut data = HashMap::new();
    for entry in read_dir(entry_path).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            let line_iterator = match read_lines(&path) {
                Ok(l) => l,
                Err(_) => continue,
            };
            let mut ips = HashMap::new();
            for line in line_iterator {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => continue,
                };
                let mut split = line.split("|");
                let inner_date = match split.next() {
                    Some(d) => d,
                    None => continue,
                };
                let inner_ip = match split.next() {
                    Some(i) => i,
                    None => continue,
                };
                ips.insert(inner_date.to_string(), inner_ip.to_string());
            }
            let filename = entry.path().file_name().unwrap().to_string_lossy().to_string();
            data.insert(filename, ips);
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

/// returns a json in following format (free space in bytes):
/// ```
/// {
///     "free_space" : 2250445385728
/// }
/// ```
#[get("/get_free_space")]
pub async fn get_free_space() -> Json<i64> {
    let free_space: i64 = read_to_string(FREE_SPACE_FILE).unwrap().parse().unwrap();
    Json::from(free_space)
}


fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}