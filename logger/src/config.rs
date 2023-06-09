// SPDX-License-Identifier: BSD-3-Clause
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct AnnulsConfigPostgres {
	host: String,
	user: String,
	password: String,
	database: String,
}

#[derive(Deserialize, Debug)]
pub struct AnnulsConfigStorage {
	redis: String,
	postgres: AnnulsConfigPostgres
}

#[derive(Deserialize, Debug)]
pub struct AnnulsConfigIRCServer {
	host: String,
	username: Option<String>,
	realname: Option<String>,
	nickname: Option<String>,
	channels: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct AnnulsConfigIRC {
	username: String,
	realname: String,
	nickname: String,
	servers: Map<String, AnnulsConfigIRCServer>,
}

#[derive(Deserialize, Debug)]
pub struct AnnulsConfigServer {
	host: String
}

#[derive(Deserialize, Debug)]
pub struct AnnulsConfig {
	storage: AnnulsConfigStorage,
	irc: AnnulsConfigIRC,
	server: AnnulsConfigServer
}
