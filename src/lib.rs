#[macro_use]
extern crate diesel;
use diesel::prelude::*;

use crate::models::UpdateTodo;

use self::models::{NewTodo, Todo};
use diesel::SqliteConnection;
use uuid::Uuid;

pub mod models;
pub mod schema;

pub fn create_todo<'a>(conn: &mut SqliteConnection, text: &'a str, done: bool) -> Todo {
    use schema::todos;
    let new_todo = NewTodo {
        id: &Uuid::new_v4().to_string(),
        text,
        done,
    };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(conn)
        .expect("Error saving new post");

    Todo {
        id: new_todo.id.to_string(),
        text: new_todo.text.to_string(),
        done: new_todo.done,
    }
}

pub fn update_todo<'a>(
    conn: &mut SqliteConnection,
    uid: &'a str,
    text: Option<String>,
    done: Option<bool>,
) {
    let query = schema::todos::dsl::todos.filter(schema::todos::id.eq(uid));

    diesel::update(query)
        .set(UpdateTodo { text, done })
        .execute(conn)
        .expect("Error updating todo");
}

pub fn delete_todo<'a>(conn: &mut SqliteConnection, uid: &'a str) {
    let update = schema::todos::dsl::todos.filter(schema::todos::id.eq(uid));

    diesel::delete(update)
        .execute(conn)
        .expect("Error updating todo");
}

pub fn get_todo<'a>(conn: &mut SqliteConnection, uid: &'a str) -> Todo {
    use schema::todos::dsl::*;
    todos
        .filter(id.eq(uid))
        .first::<Todo>(conn)
        .expect("Error loading todo")
}

pub fn get_todos<'a>(
    conn: &mut SqliteConnection,
    filter_id: Option<String>,
    filter_text: Option<String>,
    filter_done: Option<bool>,
) -> Vec<Todo> {
    use schema::todos::dsl::*;
    let mut query = todos.into_boxed();

    if let Some(filter_id) = filter_id {
        query = query.filter(id.eq(filter_id));
    }
    if let Some(filter_text) = filter_text {
        query = query.filter(text.eq(filter_text));
    }
    if let Some(filter_done) = filter_done {
        query = query.filter(done.eq(filter_done));
    }
    query.load::<Todo>(conn).expect("Error loading todos")
}
