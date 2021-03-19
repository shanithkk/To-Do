use rocket::{self, get};
use crate::controller::todo::{create_todo,get_todos, check_todo};
use crate::controller::todo::DbConnection;
use crate::models::todo::Todo;
use crate::models::todo::NewTodo;
//use serde_json::Value;
use rocket_contrib::json::Json;
//use rocket::{self, get, post};

#[get("/")]
pub fn get_todo(db_conn: DbConnection)->Json<Vec<Todo>>{
    let result= get_todos(db_conn);
    result
}

#[post("/", data = "<new_todo>")]
pub fn create_todos(db_conn: DbConnection,new_todo: Json<NewTodo>)->Json<Todo>{
    create_todo(db_conn,new_todo)
}

#[put("/<id>")]
pub fn check(conn: DbConnection, id :i32)->Json<Todo>{
    check_todo(conn, id)
}