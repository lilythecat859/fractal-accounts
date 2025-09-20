use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;
use tracing::info;

mod grpc;
use crate::grpc::ledger_server::LedgerServer;
use crate::grpc::LedgerService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("ledger_svc=debug").init();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let addr = "[::]:50053".parse()?;
    let svc = LedgerService { db };
    info!("ledger-svc gRPC on {}", addr);
    Server::builder()
        .add_service(LedgerServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
