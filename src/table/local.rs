// Example of how to local

use anyhow::Result;
use serde::Serialize;
use spin_sdk::{
    http::{IntoResponse, Request, Response},
    http_component,
    sqlite::{Connection, Value},
};

pub async fn example() -> Result<()> {
    let connection = Connection::open_default()?;

    let execute_params = [
        Value::Text("Try out Spin SQLite".to_owned()),
        Value::Text("Friday".to_owned()),
    ];
    connection.execute(
        "INSERT INTO todos (description, due) VALUES (?, ?)",
        execute_params.as_slice(),
    )?;

    let rowset = connection.execute("SELECT id, description, due FROM todos", &[])?;

    let todos: Vec<_> = rowset
        .rows()
        .map(|row| ToDo {
            id: row.get::<u32>("id").unwrap(),
            description: row.get::<&str>("description").unwrap().to_owned(),
            due: row.get::<&str>("due").unwrap().to_owned(),
        })
        .collect();

    let body = serde_json::to_vec(&todos)?;

    Ok(())
}

#[derive(Serialize)]
struct ToDo {
    id: u32,
    description: String,
    due: String,
}
