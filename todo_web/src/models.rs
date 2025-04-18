use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Todo {
    pub fn create_from_param(param: TodoCreationParam, id: i32) -> Todo {
        Todo {
            id,
            title: param.title,
            description: param.description,
            completed: param.completed,
        }
    }

    pub fn update_from_param(&mut self, param: TodoEditionParam) {
        if let Some(title) = param.title {
            self.title = title;
        }

        if let Some(description) = param.description {
            self.description = description
        }

        if let Some(completed) = param.completed {
            self.completed = completed
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoCreationParam {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoEditionParam {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
