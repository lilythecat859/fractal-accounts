use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;
    info!("running migrations…");
    sqlx::migrate!("../../migrations").run(&pool).await?;
    info!("migrations complete");
    Ok(())
}
