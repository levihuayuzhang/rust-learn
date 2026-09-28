use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UppdateTask {
    pub title: Option<String>,
    pub done: Option<bool>,
}
