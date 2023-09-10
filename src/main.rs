use fixedfloat::error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = fixedfloat::configuration::get_config().expect("Failed to read configuration.");
    let pool = sqlx::PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database.");

    let host = config.application.host;
    let port = config.application.port;

    fixedfloat::run(host, port, pool).await?;

    Ok(())
}
