use actix_web::{web::{Data,Json}, HttpResponse, Responder,post,get};
use serde::{Deserialize, Serialize};

use sqlx::FromRow;
use sqlx::Error;
use sqlx::MySqlPool;

#[derive(Debug, FromRow,Serialize, Deserialize)]
struct CreateNewTodo {
    title: String,
    description: Option<String>,
}
#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    title: String,
    description: Option<String>,
    status:String,
}
#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct Todos {
    id: i32,
    title: String,
    description: Option<String>,
}



#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct ErrorResponse {
    error: String,
}


#[post("/todo")]
async fn create_todo(
    pool: Data<MySqlPool>,
    body: Json<CreateNewTodo>)-> impl Responder {
        let response=sqlx::query("insert into todo (title,description) values (?,?)")
        .bind(&body.title)
        .bind(&body.description)
        .execute(&**pool)
        .await;

    match response {
        Ok(_id) => {
            HttpResponse::Ok().json(Todo{
                id: _id.last_insert_id() as i32,
                title: body.title.clone(),
                description: body.description.clone(),
                status: "created".to_string()
            })
        },
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse{error:e.to_string()}),
    }
        
}


#[get("/todos")]
pub async fn get_todos(pool: Data<MySqlPool>) -> impl Responder {
    let todos = sqlx::query_as::<_,Todos>("select * from todo")
        .fetch_all(&**pool)
        .await;

    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse{error:e.to_string()}),
    }
}