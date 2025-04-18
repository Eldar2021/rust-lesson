use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Html,
    Json,
};

use crate::{
    models::{Todo, TodoCreationParam, TodoEditionParam},
    utils::fs_helper::{delete, read, update, write},
};

pub async fn root() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

pub async fn get_todos(Query(params): Query<HashMap<String, String>>) -> Json<Vec<Todo>> {
    let todos = read(Some(params));

    Json(todos)
}

pub async fn post_todo(Json(todo): Json<TodoCreationParam>) -> StatusCode {
    write(todo);

    StatusCode::CREATED
}

pub async fn put_todo(Path(id): Path<i32>, Json(todo): Json<TodoEditionParam>) -> StatusCode {
    update(id, todo);

    StatusCode::CREATED
}

pub async fn delete_todo(Path(id): Path<i32>) -> StatusCode {
    delete(id);

    StatusCode::OK
}
