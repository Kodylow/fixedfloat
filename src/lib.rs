pub mod configuration;
pub mod error;
pub mod routes;
pub use routes::*;
pub mod startup;

use startup::init_app;
use std::net::TcpListener;

pub async fn run(host: String, port: u16, pool: sqlx::PgPool) -> Result<(), anyhow::Error> {
    let app = init_app(pool)?;
    let addr = TcpListener::bind((host, port))?.local_addr()?;
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        });

    println!("Listening on {}", addr);

    server.await.map_err(|error| error.into())
}
