use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    status: String,
}

impl Status {
    pub fn ok() -> Self {
        Status {
            status: "ok".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "lists")]
pub struct List {
    pub id: i32,
    pub title: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "tasks")]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub is_done: bool,

    #[allow(dead_code)]
    #[serde(skip)]
    list_id: i32,
}

#[derive(Deserialize)]
pub struct NewTask {
    pub title: String,
}

#[derive(Serialize)]
pub struct TaskId {
    pub id: i32,
}
