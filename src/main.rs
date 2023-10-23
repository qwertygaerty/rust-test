use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::put;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    id: u32,
    name: String,
}

// Shared mutable state with thread safety
struct AppState {
    products: Mutex<Vec<Product>>,
}

#[get("/products")]
async fn get_products(data: web::Data<AppState>) -> HttpResponse {
    let products = data.products.lock().unwrap();
    HttpResponse::Ok().json(&*products)
}

#[post("/product")]
async fn create_product(
    product: web::Json<Product>,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("Creating product: {:?}", product);

    data.products.lock().unwrap().push(product.0);

    HttpResponse::Ok().json(&data.products)
}


#[get("/product/{id}")]
async fn read_product(info: web::Path<(u32,)>) -> impl Responder {
    println!("Retrieving product with id: {:?}", info);
    HttpResponse::Ok().body(format!("Retrieved product id: {:?}", info.0))
}

#[put("/product/{id}")]
async fn update_product(id: web::Path<u32>, product: web::Json<Product>) -> impl Responder{
    println!("Updating product with id: {:?} to {:?}", id, product);
    HttpResponse::Ok().body(format!("Updated product id: {:?}", id))
}

#[delete("/product/{id}")]
async fn delete_product(info: web::Path<(u32,)>) -> impl Responder {
    println!("Deleting product with id: {:?}", info);
    HttpResponse::Ok().body(format!("Deleted product id: {:?}", info.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        products: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(create_product)
            .service(read_product)
            .service(update_product)
            .service(delete_product)
            .service(get_products)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}