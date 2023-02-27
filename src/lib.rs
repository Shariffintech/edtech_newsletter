use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};


async fn health_check(_req: HttpRequest) -> impl Responder {
   
    HttpResponse::Ok() 
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub async fn run() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(greet))
                .route("/{name}", web::get().to(greet))
                .route("/health_check", web::get().to(health_check))
            
        })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}