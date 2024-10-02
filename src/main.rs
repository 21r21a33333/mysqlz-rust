use actix_web::{dev::Path, get, http::StatusCode, web::{self, Data, Json}, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

mod routes;
use routes::*;

mod database;
use database::*;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = database_connection().await.expect("Failed to create dbpool");
    println!("Connected to database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .service(index)
            .service(hello)
            .service(create_new_user)
            .service(create_todo)
            .service(get_todos)
            
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
    
}