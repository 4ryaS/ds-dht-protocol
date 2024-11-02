use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod api { // Declare the parent module
    pub mod node; // Declare the submodule
}
mod models {
    pub mod node;
}

use api::node::{add_node, get_nodes, AppState};
use models::node::Node;                 
use std::sync::Mutex;

// health check handler
async fn health_check() -> impl Responder {
    return HttpResponse::Ok().body("DHT server is running");
}

// async main function with actix web runtime
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // shared state
    let app_state = web::Data::new(AppState {
        nodes: Mutex::new(Vec::new())
    });

    // starting the http server on port 5000
    HttpServer::new(move || {
        App::new()
            // defining the health check route
            .route("/", web::get().to(health_check))
            .app_data(app_state.clone())
            .service(add_node)
            .service(get_nodes)
    })
    .bind("localhost:5000")?
    .run()
    .await
}