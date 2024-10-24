use actix_web::{web, App, HttpServer};
use std::io;
use std::ptr::null;
use log::{error, info}; // logging macros
use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};
use WebApi::{config, constants, routes, redis_server::RedisClient, models};
use WebApi::models::BookA;

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

    let redis_client = RedisClient::new("redis://127.0.0.1:6379/")
        .await
        .expect("Failed to create Redis client");

    // Wrap the RedisClient in web::Data for shared access
    let redis_data = web::Data::new(redis_client);



    let book = BookA{
        title: "Land before time".to_string(),
        author: "Sebastian N".to_string(),
        pages: 100,
        description: None,
    };

    println!("The book: {:?}",book);

    let server_config = config::AppConfig::from_config_file(constants::CONFIG_FILE)
        .map_err(|e| {
            error!("Failed to load config: {}", e);
            io::Error::new(io::ErrorKind::Other, format!("ConfigError: {}", e))
        })?;

    info!("Config loaded successfully: {:?}", server_config);

    let ip = server_config.server.ip;
    let port = server_config.server.port;
    let connection_str = format!("{}:{}", ip, port);

    // Log server start info
    info!("Starting HTTP server at {}",connection_str);

    // Add the `move` keyword here
    let server = HttpServer::new(move || {
        App::new()
            .app_data(redis_data.clone())
            .configure(routes::init_routes)
    })
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
