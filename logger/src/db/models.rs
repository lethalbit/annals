// SPDX-License-Identifier: BSD-3-Clause

use crate::db::schema;

use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::irc_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
	pub id: i32,
	pub nickname: String,
	pub username: Option<String>,
	pub first_seen: NaiveDateTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::irc_servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Server {
	pub id: i32,
	pub server_name: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::irc_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Log {
	pub id: i32,
	pub channel: String,
	pub ts: NaiveDateTime,
	pub user_id: i32,
	pub server_id: i32,
	pub raw_line: String,
}
