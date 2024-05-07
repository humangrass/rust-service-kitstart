mod api;
mod config;
mod entries;
mod handler;

use log::info;
use std::process;

use crate::config::TemplateConfig;
use crate::handler::HTTPHandler;
use cli::setup_cli;
use database::postgres::new_postgres_pool;
use logger::tracer_logger::new_tracer_logger;

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

    let config = TemplateConfig::new(&cli.config)?;
    new_tracer_logger(cli.log_level);

    let pool = new_postgres_pool(config.database)
        .await
        .expect("🪂 Failed to create Postgres pool");
    let handler = HTTPHandler::new(pool);

    let bind_address = (config.template.host, config.template.port);
    info!("Bind address: {}:{}", bind_address.0, bind_address.1);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    axum::serve(listener, handler.mount_routes()).await.unwrap();

    Ok(())
}
