use std::sync::Arc;

use anyhow::Result;
use database::database_connection;
use tokio::net::TcpListener;

mod database;
mod router;
mod service;
mod model;
mod middleware;

#[tokio::main]
async fn main() -> Result<()> {
    let db = Arc::new(database_connection());
    let app = router::all_router(db);
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
     axum::serve(listener, app).await?;
    Ok(())
}
