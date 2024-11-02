use actix_web::{post, get, web, HttpResponse, Responder};
use crate::models::node::Node;
use std::sync::Mutex;

// shared state to store nodes
pub struct AppState {
    pub nodes: Mutex<Vec<Node>>,
}

// POST route to add a new node
#[post("/nodes")]
async fn add_node(data: web::Data<AppState>, new_node: web::Json<Node>) -> impl Responder {
    let mut nodes = data.nodes.lock().unwrap();
    // Add new node to the vector
    nodes.push(new_node.into_inner());
    return HttpResponse::Ok().json("Node added successfully");
}

// GET route to retrieve all nodes
#[get("/nodes")]
async fn get_nodes(data: web::Data<AppState>) -> impl Responder {
    let nodes = data.nodes.lock().unwrap();
    return HttpResponse::Ok().json(&*nodes);
}