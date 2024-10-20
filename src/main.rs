use actix_web::{web, App, HttpServer};
use std::io;
use log::{error, info}; // logging macros
use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};


mod routes;
mod models;
mod config;
mod constants;

#[actix_web::main]
async fn main() -> io::Result<()> {

    // Initialize logger with flexi_logger to log to a file
    let _logger = Logger::try_with_str("info") // log level set to info and above (error, warn)
        .unwrap()
        .log_to_file(FileSpec::default().directory(constants::LOG_FOLDER).basename(constants::LOG_PREFIX))
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();


    let server_config = config::AppConfig::from_config_file(constants::CONFIG_FILE)
        .map_err(|e| {
            error!("Failed to load config: {}", e);
            io::Error::new(io::ErrorKind::Other, format!("ConfigError: {}", e))
        })?;

    info!("Config loaded successfully: {:?}", server_config);

    let ip = server_config.server.ip;
    let port = server_config.server.port;
    let connection_str = format!("{}:{}",ip, port);

    // Log server start info
    info!("Starting HTTP server at {}",connection_str);

    let server = HttpServer::new(|| App::new().configure(routes::init_routes))
        .workers(4)
        .bind(&connection_str)
        .map_err(|e| {
            error!("Failed to bind server to address: {}", e);
            e
        })?;

    info!("Server bound successfully to {}", connection_str);

    server.run().await
        .map_err(|e| {
        error!("Error running the server: {}", e);
        e
    })

}
