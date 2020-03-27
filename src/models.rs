use super::schema::todos;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Deserialize, Serialize, Queryable, PartialEq, Debug)]
#[table_name = "todos"]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
}
