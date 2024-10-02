use actix_web::{http::StatusCode, post, web::{self,Json}, HttpResponse, Responder};
use crate::routes::logging;
use crate::routes::hello_user::User;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
struct CreateUser{
    id: i32,
    user:User
}
#[post("/create")]
pub async fn create_new_user(user: Json<User>) -> impl Responder {
    println!("{:?}", user.0);
    logging("POST /create");    
    (Json(CreateUser{id: 1, user: user.0}), StatusCode::CREATED)
}