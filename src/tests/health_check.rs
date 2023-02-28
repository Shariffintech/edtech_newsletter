//actix_rt::test is a test framework for actix-web
// with it you dont need to use the #[test] attribute

// inspect what code gets generated with
// `cargo expand --test health_check`

use edtech_newsletter::run;
use actix_web::dev::Server;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("failed to execute request");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

async fn spawn_app() -> Result<Server, std::io::Error>{
    edtech_newsletter::run().await
}
