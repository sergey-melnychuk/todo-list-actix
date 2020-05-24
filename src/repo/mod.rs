use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::model::{List, Task, TaskId};

pub async fn get_lists(client: &deadpool_postgres::Client) -> io::Result<Vec<List>> {
    let stmt = client.prepare("select * from lists order by id desc").await.unwrap();

    let lists = client.query(&stmt, &[])
        .await
        .expect("Failed to query lists")
        .iter()
        .map(|row| List::from_row_ref(row).unwrap())
        .collect::<Vec<List>>();

    Ok(lists)
}

pub async fn get_tasks(client: &deadpool_postgres::Client, list_id: i32) -> io::Result<Vec<Task>> {
    let stmt = client
        .prepare("select * from tasks where list_id = $1 order by id")
        .await
        .unwrap();

    let tasks = client.query(&stmt, &[&list_id])
        .await
        .expect("Failed to query tasks")
        .iter()
        .map(|row| Task::from_row_ref(row).unwrap())
        .collect::<Vec<Task>>();

    Ok(tasks)
}

pub async fn get_one_task(client: &deadpool_postgres::Client, list_id: i32, task_id: i32) -> io::Result<Task> {
    let stmt = client
        .prepare("select * from tasks where list_id = $1 and id = $2")
        .await
        .unwrap();

    let row = client.query_one(&stmt, &[&list_id, &task_id])
        .await
        .expect("Failed to query one task");

    let task = Task::from_row_ref(&row).unwrap();

    Ok(task)
}

pub async fn create_task(client: &deadpool_postgres::Client, list_id: i32, title: &str) -> io::Result<TaskId> {
    let stmt = client
        .prepare("insert into tasks (title, list_id) values ($1, $2) returning id")
        .await
        .unwrap();

    let row = client.query_one(&stmt, &[&title.to_string(), &list_id])
        .await
        .expect("Failed to create a task");

    let id: i32 = row.get(0);

    Ok(TaskId { id })
}

pub async fn check_task(client: &deadpool_postgres::Client, list_id: i32, task_id: i32) -> io::Result<()> {
    let stmt = client
        .prepare("update tasks set is_done = true where id = $1 and list_id = $2")
        .await
        .unwrap();

    client.execute(&stmt, &[&task_id, &list_id])
        .await
        .expect("Failed to mark task as checked");

    Ok(())
}

pub async fn delete_task(client: &deadpool_postgres::Client, list_id: i32, task_id: i32) -> io::Result<()> {
    let stmt = client
        .prepare("delete from tasks where id = $1 and list_id = $2")
        .await
        .unwrap();

    client.execute(&stmt, &[&task_id, &list_id])
        .await
        .expect("Failed to delete task");

    Ok(())
}
