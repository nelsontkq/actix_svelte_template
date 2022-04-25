use super::schema::todos;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub id: &'a str,
    pub text: &'a str,
    pub done: bool,
}
#[derive(AsChangeset)]
#[table_name = "todos"]
pub struct UpdateTodo {
    pub text: Option<String>,
    pub done: Option<bool>,
}
