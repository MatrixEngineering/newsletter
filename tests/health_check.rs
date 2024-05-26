//! tests/health_check.rs

#[tokio::test]
async fn health_check_works() {
    // No .await, no .expect
    spawn_app();
}

fn spawn_app() {
    let server = zero2prod::run("127.0.0.1:0").expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
