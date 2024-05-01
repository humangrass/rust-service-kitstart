mod config;
mod handler;
mod api;
mod entries;

use std::process;
use log::info;

use cli::setup_cli;
use database::postgres::new_postgres_pool;
use logger::tracer_logger::new_tracer_logger;
use crate::config::TemplateConfig;
use crate::handler::HTTPHandler;


#[tokio::main]
async fn main() {
    println!("*-*-*\tHTTP REST API Service Template. Use it however you want!\t*-*-*");
    if let Err(err) = run().await {
        eprintln!("Failed to run the application: {}", err);
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = setup_cli();

    let config = TemplateConfig::new(Some(cli.config.ok_or("Config file path not provided")?))?;
    new_tracer_logger(cli.log_level);

    let pool = new_postgres_pool(config.database).await.expect("🪂 Failed to create Postgres pool");
    let handler = HTTPHandler::new(pool);

    let bind_address = format!("{}:{}", config.template.host, config.template.port);
    info!("Bind address: {bind_address}");
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    axum::serve(listener, handler.mount_routes())
        .await
        .unwrap();

    Ok(())
}
