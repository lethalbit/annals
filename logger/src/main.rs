// SPDX-License-Identifier: BSD-3-Clause

use tracing::{
	trace, debug, info, warn, error
};
use tracing_subscriber::{
	filter::LevelFilter, fmt, prelude::*
};
use clap::{
	arg, ArgAction, Command
};
use axum::{
	Router, Server,
	routing::get,
	extract::{
		ws::{Message, WebSocket},
		Path, State, WebSocketUpgrade
	},
	response::IntoResponse
};

use diesel::{
	PgConnection, Connection, ConnectionError
};



use tokio;
use irc;
use chrono;
use tokio_stream::StreamExt;
use serde_json::json;

#[cfg(tokio_unstable)]
use console_subscriber;

use std::{
	fs::{
		File
	}, io::{
		BufReader
	}
};


pub mod config;
pub mod db;

#[derive(Debug, Clone)]
struct IRCLogMessage {
	msg: irc::proto::Message,
	server: String,
	time: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
struct ServiceState {
	msgs: tokio::sync::broadcast::Sender<IRCLogMessage>,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
	let mtch = Command::new("annals")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(
			arg!(-c <config>)
			.default_value("./annuls.json")
			.help("Annuls configuration file")
		)
		.arg(
			arg!(-v --verbose)
			.help("Enable Debug Logging, specify more than once for trace logging.")
			.action(ArgAction::Count)
		)
		.arg(
			arg!(-m --migrate)
			.help("Run annuls log database migrations")
			.action(ArgAction::SetTrue)
		)
		.get_matches();

	let subscriber = tracing_subscriber::registry()
		.with(fmt::layer())
		.with(match mtch.get_count("verbose") {
			2 => LevelFilter::TRACE,
			1 => LevelFilter::DEBUG,
			_ => LevelFilter::INFO
		});

	if cfg!(tokio_unstable) {
		subscriber.with(console_subscriber::spawn()).init();
	} else {
		subscriber.init();
	}

	debug!("Reading Configuration file");
	let cfg_file = File::open(
		mtch.get_one::<String>("config").expect("Missing config file")
	).unwrap();
	let reader = BufReader::new(cfg_file);
	let cfg: config::AnnulsConfig = serde_json::from_reader(reader).unwrap();

	let mut db_conn = PgConnection::establish(
		format!("{}",cfg.storage.postgres).as_str()
	).unwrap_or_else(
		|_| panic!("Unable to connect to postgres server {}, is it running?", cfg.storage.postgres.host)
	);

	if mtch.get_flag("migrate") {
		info!("Running migrations for {}/{}", cfg.storage.postgres.host, cfg.storage.postgres.database);
		db::run_migrations(&mut db_conn);
		info!("Done");
		return
	}

	let (tx, _) = tokio::sync::broadcast::channel::<IRCLogMessage>(1024);

	let svc_state = ServiceState {
		msgs: tx.clone(),
	};

	let mut irc_clients = Vec::with_capacity(cfg.irc.servers.capacity());

	for srv in cfg.irc.servers {
		let mut irc_server = srv.clone();

		if irc_server.nickname.is_none() {
			irc_server.nickname = Some(cfg.irc.nickname.clone());
		}

		if irc_server.username.is_none() {
			irc_server.username = Some(cfg.irc.username.clone());
		}

		if irc_server.realname.is_none() {
			irc_server.realname = Some(cfg.irc.realname.clone());
		}


		let tx_chan = tx.clone();

		irc_clients.push(tokio::task::spawn(
			async move{
				if let Err(e) = irc_client(irc_server, tx_chan).await {
					error!("IRC Client Error: {e}");
				};
			}
		));

	}

	let _res = tokio::join!(
		futures::future::join_all(irc_clients),
		tokio::task::spawn(async {
			api_server(cfg.server, svc_state).await;
		}),
		tokio::task::spawn(async move {
			if let Err(e) = redis_cache(cfg.storage.redis.clone(), tx.subscribe()).await {
				error!("redis cache error: {e}");
			}
		}),
		tokio::task::spawn(async {
			collect_stats(cfg.storage.postgres).await
		})
	);

}

#[tracing::instrument(skip_all, name = "irc client")]
async fn irc_client(cfg: config::AnnulsConfigIRCServer, tx: tokio::sync::broadcast::Sender<IRCLogMessage>) -> Result<(), irc::error::Error>{
	info!("Starting IRC client for {} ({}:{})", cfg.name, cfg.host, cfg.port);
	info!("[{}] * User: {}", cfg.name, cfg.username.as_ref().unwrap());
	info!("[{}] * Nick: {}", cfg.name, cfg.nickname.as_ref().unwrap());
	info!("[{}] * Chan: {}", cfg.name, cfg.channels.len());

	let irc_config = irc::client::prelude::Config {
		nickname: cfg.nickname,
		username: cfg.username,
		realname: cfg.realname,
		server: Some(cfg.host),
		port: Some(cfg.port),
		channels: cfg.channels,
		..irc::client::prelude::Config::default()
	};

	let mut client = irc::client::prelude::Client::from_config(
		irc_config
	).await?;

	client.identify()?;

	let mut stream = client.stream()?;

	while let Some(msg) = stream.next().await.transpose()? {
		trace!("[{}] {:#?} {:#?}",cfg.name, msg.source_nickname(), msg.command);
		match msg.command {
			_ => ()
		};

		if let Err(e) = tx.send(IRCLogMessage {
			msg: msg.clone(), server: cfg.name.clone(), time: chrono::offset::Utc::now()
		}) {
			error!("[{}] Unable to send message: {}", cfg.name, e);
			return Err(irc::error::Error::AsyncChannelClosed);
		}

	}

	Ok(())

}

#[tracing::instrument(skip_all, name = "api server")]
async fn api_server(cfg: config::AnnulsConfigServer, svc_state: ServiceState) {
	let api = Router::new()
		.route("/api/live/:server/:channel", get(api_realtime_log))
		.route("/api/log/:server/:channel/:year/:month/:day", get(api_historic_log))
		.with_state(svc_state.clone());


	let server = Server::bind(&cfg.host.parse().unwrap())
		.serve(api.into_make_service());

	info!("Starting API Endpoint on http://{}", server.local_addr());
	server.await.unwrap();
}

#[axum::debug_handler]
async fn api_realtime_log(
	ws: WebSocketUpgrade, Path((server, channel)): Path<(String, String)>,
	State(svc_state): State<ServiceState>
) -> impl IntoResponse {
	debug!("New websocket client for {}/{}", server, channel);

	ws.on_upgrade(|ws: WebSocket| async {
		feed_realtime_api(svc_state, ws, server, channel).await
	})
}

async fn feed_realtime_api(svc_state: ServiceState, mut ws: WebSocket, server: String, chan: String) {
	let mut rx = svc_state.msgs.subscribe();

	while let Ok(irc_msg) = rx.recv().await {
		if irc_msg.server.eq(&server) {
			if let irc::proto::Command::PRIVMSG(ref tgt, ref m) = irc_msg.msg.command {
				if chan.eq(tgt) {
					ws.send(Message::Text(json!({
						"server": server.clone(),
						"channel": tgt.clone(),
						"message": m.clone(),
						"timestamp": irc_msg.time.timestamp_millis(),
						"raw": irc_msg.msg.to_string(),
						"nick": irc_msg.msg.source_nickname(),
					}).to_string())).await.unwrap();
				}
			}
		}
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;
	}
}

async fn api_historic_log(
	Path((server, channel, year, month, day)): Path<(String, String, u16, u8, u8)>,
	State(svc_state): State<ServiceState>
) -> impl IntoResponse {

}

#[tracing::instrument(skip_all, name = "redis client")]
async fn redis_cache(host: String, mut rx: tokio::sync::broadcast::Receiver<IRCLogMessage>) -> redis::RedisResult<()> {
	info!("Starting REDIS cache ventilator");
	let client = redis::Client::open(format!("redis://{host}").as_str())?;
	let mut con = client.get_tokio_connection_manager().await?;

	while let Ok(msg) = rx.recv().await {

	}

	Ok(())
}

#[tracing::instrument(skip_all, name = "stats collector")]
async fn collect_stats(cfg: config::AnnulsConfigPostgres) {
	info!("Starting stats collector");

	loop {
		tokio::time::sleep(std::time::Duration::from_secs(5 * 60)).await;
		info!("Collecting Statistics...");

	}
}
