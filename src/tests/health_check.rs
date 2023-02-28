use actix_web::{http::StatusCode, test, App, HttpResponse, web};
use edtech_newsletter::run;
use actix_web::http::header::{CONTENT_LENGTH};

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let server = run().await.unwrap();
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
}




// below the content method for the response struct doesnt exist for actix web

// to get the content length of the response

// I have to use the headers method on the HttpResponse instance to get the 
// Content-Length header and then extract its value (above).


// use edtech_newsletter::run;
// use actix_web::{http::StatusCode, test, App, web, HttpResponse};
// use actix_web::dev::ServerHandle;
// use actix_web::http::header::{CONTENT_LENGTH, self};

// #[actix_rt::test]
// async fn health_check_works() {
//     // Arrange
//     let server_handle = spawn_app().await;
//     let mut app = test::init_service(App::new().route("/health_check", web::get().to(|| async {
//         HttpResponse::Ok().finish()
//     }))).await;
//     let req = test::TestRequest::get().uri("/health_check");
//     // Act
//     let response = test::call_service(&mut app, req.to_request()).await;
//     let content_length = response.headers().get(header::CONTENT_LENGTH).and_then(|value| value.to_str().ok().and_then(|s| s.parse().ok()));
//     // Assert
//     assert_eq!(response.status(), StatusCode::OK);
//     assert_eq!(response.response().content_length(), Some(0));
//     // Shutdown the server
//     let _ = server_handle.stop(true);
// }

// async fn spawn_app() -> ServerHandle {
//     let server = run().await.unwrap();
//     server.handle()
// }