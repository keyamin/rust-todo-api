#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Todo {
    id: String,
    title: String,
    is_done: bool,
}

impl Todo {
    fn new(title: String, is_done: bool) -> Self {
        Todo {
            id: String::from(""),
            title,
            is_done,
        }
    }
}

#[derive(Deserialize)]
struct PostTodo {
    title: String,
}

#[get("/todos")]
fn get_todo() -> String {
    format!("{:?}", Todo::new(String::from("todo"), false))
}

#[post("/todos", format = "application/json", data = "<todo>")]
fn post_todo(todo: Json<PostTodo>) -> Json<Todo> {
    let incoming_todo = todo.into_inner();
    Json(Todo::new(incoming_todo.title, false))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_todo, post_todo])
}
