use actix_web::{http::StatusCode, test, App, HttpResponse, web};
use edtech_newsletter::run;
use actix_web::http::header::{CONTENT_LENGTH};
use std::net::TcpListener;
// use actix_web::dev::ServerHandle;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let server = spawn_app().await;
    let mut app = test::init_service(App::new().route("/health_check", web::get().to(|| async {
        HttpResponse::Ok().append_header((CONTENT_LENGTH, 0)).finish()
    }))).await;
    let req = test::TestRequest::get().uri("/health_check");
    // Act
    let response = test::call_service(&mut app, req.to_request()).await;
    let content_length = response.headers().get(CONTENT_LENGTH).and_then(|value| value.to_str().ok().and_then(|s| s.parse().ok()));
    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(content_length, Some(0));
    // Shutdown the server
    drop(server);
    // let _ = server.stop(true).await;
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);   
    // return the server handle to the caller
    format!("127.0.0.1:{}", port)
}





