use crate::routes;

use axum::{routing::get, Router};

pub fn init_app(pool: sqlx::PgPool) -> Result<Router, anyhow::Error> {
    let app = Router::new()
        .route("/", get(routes::health_check))
        .with_state(pool);

    Ok(app)
}
