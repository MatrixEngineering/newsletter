//! src/main.rs
use zero2prod::configuration::get_configuration;
use zero2prod::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration. ");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = std::net::TcpListener::bind(address).expect("Failed to bind address Port");
    run(listener)?.await
}
