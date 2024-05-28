//! src/main.rs
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration. ");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres.");

    let listener = std::net::TcpListener::bind(address).expect("Failed to bind address Port.");
    run(listener, connection_pool)?.await
}
