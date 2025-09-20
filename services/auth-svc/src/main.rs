use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::info;

mod grpc;
mod handlers;
mod jwt;

use crate::grpc::auth_server::AuthServer;
use crate::grpc::AuthService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("auth_svc=debug").init();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    sqlx::migrate!("../../migrations").run(&db).await?;

    // gRPC
    let addr = SocketAddr::from(([0, 0, 0, 0], 50051));
    let svc = AuthService { db: db.clone() };
    info!("gRPC auth-svc on {}", addr);
    tokio::spawn(async move {
        Server::builder()
            .add_service(AuthServer::new(svc))
            .serve(addr)
            .await
            .unwrap();
    });

    // REST gateway on 8080
    let app = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .with_state(db);

    let gw = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("REST gateway on {}", gw);
    axum::Server::bind(&gw)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
