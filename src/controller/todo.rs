//use crate::schema::todo;
use crate::models::todo::{Todo, NewTodo};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::prelude::*;
use diesel::QueryDsl;
//use serde_json::Value;
//use diesel::pg::PgConnection;
use crate::schema::todo;

#[database("postgres")]
pub struct DbConnection(PgConnection);

pub fn create_todo(conn: DbConnection ,new_todo: Json<NewTodo>)-> Json<Todo>{
    let result = diesel::insert_into(todo::table)
        .values(&*new_todo)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

pub fn get_todos(conn: DbConnection) -> Json<Vec<Todo>> {
    let todos = todo::table
        .order(todo::columns::id.desc())
        .load::<Todo>(&*conn)
        .unwrap();
    Json(todos)
}

pub fn check_todo(conn :DbConnection, id:i32)-> Json<Todo>{
    let target =todo::table
        .filter(todo::columns::id.eq(id));
    let result = diesel::update(target)
        .set(todo::columns::checked.eq(true))
        .get_result(&*conn)
        .unwrap();
    
    Json(result)
}