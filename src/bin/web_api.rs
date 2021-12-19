// - STD
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

// - external
use rocket::serde::{
    Serialize,
    Deserialize,
    json::Json,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatecontrolData {
    user: String,
    timestamp: i64
}

/// returns a json in following format:
/// {
///     [
///         "user" : "bk123456xy",
///         "timestamp" : 1619698026
///     ]
/// }
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