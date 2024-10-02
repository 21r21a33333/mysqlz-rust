use actix_web::{dev::Path, get, http::StatusCode, web::{self, Json}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use crate::routes::logging;


#[derive(Serialize,Deserialize)]
#[derive(Debug)]
pub struct User{
    firstname: String,
    lastname: String
}

 impl User{
    fn new(firstname: String, lastname: String) -> Self{
        User{
            firstname,
            lastname
        }
    }
}


#[get("/hey/{firstname}/{lastname}")]
pub async fn hello(params: web::Path<(String, String)>) -> impl Responder {
    logging(format!("GET /hey/{}/{}", params.0.clone(), params.1.clone()).as_str());
    let user = User::new(params.0.clone(), params.1.clone());
    (Json(user),StatusCode::OK)
}

