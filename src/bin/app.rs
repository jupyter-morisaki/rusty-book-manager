use anyhow::Error;
use anyhow::Result;
use axum::routing::Router;
use tokio::net::TcpListener;

use std::net::Ipv4Addr;
use std::net::SocketAddr;

use adapter::database::connect_database_with;
use api::route::book::build_book_routers;
use api::route::health::build_health_check_routers;
use registry::AppRegistry;
use shared::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::build()?;
    let pool = connect_database_with(&app_config.database);
    let registry = AppRegistry::new(pool);

    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
