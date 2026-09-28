use crate::models::Task;
use std::{
    sync::{Arc, Mutex},
    vec,
};

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_id: Arc<Mutex<u32>>,
}

impl AppState {
    pub fn new() -> Self {
        let seed = vec![
            Task {
                id: 1,
                title: "Learn Axum".to_string(),
                done: false,
            },
            Task {
                id: 2,
                title: "Write a script".to_string(),
                done: false,
            },
            Task {
                id: 3,
                title: "Record a video".to_string(),
                done: false,
            },
        ];

        Self {
            tasks: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(4)),
        }
    }
}
