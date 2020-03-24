use super::schema::todos;
use serde::Deserialize;

#[derive(Queryable, PartialEq, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize, Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
}
