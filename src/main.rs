//! src/main.rs
use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("Zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration. ");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let connection_pool =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to postgres.");

    let listener = std::net::TcpListener::bind(address).expect("Failed to bind address Port.");
    run(listener, connection_pool)?.await
}
