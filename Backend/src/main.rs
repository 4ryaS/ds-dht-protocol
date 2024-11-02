use actix_web::{web, App, HttpServer, Responder, HttpResponse};

// health check handler
async fn health_check() -> impl Responder {
    return HttpResponse::Ok().body("DHT server is running");
}

// async main function with actix web runtime
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // starting the http server on port 5000
    HttpServer::new(|| {
        App::new()
            // defining the health check route
            .route("/", web::get().to(health_check))
    })
    .bind("localhost:5000")?
    .run()
    .await
}
