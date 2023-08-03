// SPDX-License-Identifier: BSD-3-Clause

use std::fmt;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AnnulsConfigPostgres {
	pub host: String,
	pub user: String,
	pub password: String,
	pub database: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AnnulsConfigStorage {
	pub redis: String,
	pub postgres: AnnulsConfigPostgres
}

#[derive(Deserialize, Debug, Clone)]
pub struct AnnulsConfigIRCServer {
	pub name: String,
	pub host: String,
	pub port: u16,
	pub username: Option<String>,
	pub realname: Option<String>,
	pub nickname: Option<String>,
	pub channels: Vec<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct AnnulsConfigIRC {
	pub username: String,
	pub realname: String,
	pub nickname: String,
	pub servers: Vec<AnnulsConfigIRCServer>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AnnulsConfigServer {
	pub host: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct AnnulsConfig {
	pub storage: AnnulsConfigStorage,
	pub irc: AnnulsConfigIRC,
	pub server: AnnulsConfigServer
}

impl fmt::Display for AnnulsConfigPostgres {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		// postgres://{}:{}@{}/{}
		fmt.write_str("postgres://")?;
		fmt.write_str(&self.user)?;
		fmt.write_str(":")?;
		fmt.write_str(&self.password)?;
		fmt.write_str("@")?;
		fmt.write_str(&self.host)?;
		fmt.write_str("/")?;
		fmt.write_str(&self.database)?;
		Ok(())
	}
}
