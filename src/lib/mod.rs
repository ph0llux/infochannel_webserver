// - extern crates
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
use rocket_sync_db_pools::rusqlite;
use rocket::serde::{
    Serialize,
    Deserialize,
};

// - modules
mod web_api;
pub mod constants;

// - re-exports
pub use web_api::*;

// - internal
use crate::constants::*;

#[database("news")]
pub struct NewsDatabase(rusqlite::Connection);
	
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewsFeed {
	timestamp: i64,
	headline: String,
	content: String,
	status: i64,
}

impl NewsFeed {
	pub fn add_feed<H: Into<String>, C: Into<String>>(conn: &rusqlite::Connection, timestamp: i64, headline: H, content: C, status: i64) -> Result<usize, rocket_sync_db_pools::rusqlite::Error>{
		conn.execute(SQL_ADD_NEWSFEED, rusqlite::params![timestamp, headline.into(), content.into(), status])
	}
	pub fn delete_feed_by_timestamp<H: Into<String>, C: Into<String>>(conn: &rusqlite::Connection, timestamp: i64) -> Result<usize, rocket_sync_db_pools::rusqlite::Error>{
		conn.execute(SQL_DEL_NEWSFEED_BY_TIMESTAMP, rusqlite::params![timestamp])
	}
	pub fn get_all_newsfeeds(conn: &rusqlite::Connection) -> Result<Vec<NewsFeed>, rocket_sync_db_pools::rusqlite::Error> {
		let mut stmt = conn.prepare(SQL_GET_NEWSFEEDS_ALL)?;
	    let mapped_rows = stmt.query_map([], |row| {
	        Ok(NewsFeed {
	            timestamp: row.get(0)?,
	            headline: row.get(1)?,
	            content: row.get(2)?,
	            status: row.get(3)?,
	        })
	    })?;
	    let mut feeds = Vec::new();
	    for row in mapped_rows {
	    	feeds.push(row.unwrap())
	    }
	    Ok(feeds)
	}

	pub fn timestamp(&self) -> i64 {
		self.timestamp
	}
	pub fn headline(&self) -> &str {
		&self.headline
	}
	pub fn content(&self) -> &str {
		&self.content
	}
}