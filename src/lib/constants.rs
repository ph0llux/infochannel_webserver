pub(crate) const DATECONTROL_FILE_DIR: &str = "assets/datecontrol";
pub(crate) const FREE_SPACE_FILE: &str = "assets/size_state/free_space";
pub(crate) const PUBLIC_IP_PATH: &str = "assets/public_ip";

//SQL_STATEMENTS;
pub(crate) const SQL_ADD_NEWSFEED: &str = include_str!("../SQL_Statements/add_newsfeed.sql");
pub(crate) const SQL_DEL_NEWSFEED_BY_TIMESTAMP: &str = include_str!("../SQL_Statements/del_newsfeed_by_timestamp.sql");
pub(crate) const SQL_GET_NEWSFEEDS_ALL: &str = include_str!("../SQL_Statements/get_all_newsfeeds.sql");