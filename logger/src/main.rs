// SPDX-License-Identifier: BSD-3-Clause

use tracing::{
    debug, info, warn, error, Level
};
use tracing_subscriber::{
    filter::LevelFilter, fmt, prelude::*
};
use clap::{
    arg, ArgAction, Command
};

#[tokio::main]
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
            .help("Enable Debug Logging")
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(match mtch.get_flag("verbose") {
            true => LevelFilter::DEBUG,
            false => LevelFilter::INFO
        })
        .init();

    let cfg = mtch.get_one::<String>("config").expect("Missing config file");

}
