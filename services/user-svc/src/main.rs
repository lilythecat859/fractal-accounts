use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;
use tracing::info;

mod grpc;
use crate::grpc::user_server::UserServer;
use crate::grpc::UserService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("user_svc=debug").init();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let addr = "[::]:50052".parse()?;
    let svc = UserService { db };
    info!("user-svc gRPC on {}", addr);
    Server::builder()
        .add_service(UserServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
