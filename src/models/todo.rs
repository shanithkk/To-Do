use crate::schema::todo;
use diesel::{Queryable, Insertable};
use serde::Serialize;
use serde::Deserialize;

#[derive(Queryable, Serialize)]
pub struct Todo{
    pub id : i32,
    pub title: String,
    pub checked: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name="todo"]
pub struct NewTodo{
    pub title :String
}