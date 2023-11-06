use tracing::info;

use crate::arg::parse_args;
use crate::logger::setup_logger;
use crate::web::app::{start_server, ServerConfig};

mod arg;
mod logger;
mod web;

#[tokio::main]
async fn main() {
    let cmd = parse_args();
    let matches = cmd.get_matches();
    let log_file_path = matches.get_one::<String>("log").unwrap();
    let _guard = setup_logger(log_file_path).expect("failed to set up logger");
    let bind = matches.get_one::<String>("bind").unwrap();
    let port = matches.get_one::<String>("port").unwrap();
    let server_config = ServerConfig {
        host: bind.clone(),
        port: port.clone(),
    };

    info!("starting a server");
    start_server(server_config)
        .await
        .expect("failed to start server");
}
