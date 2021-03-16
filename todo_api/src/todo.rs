use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoList {
    items: Vec<ToDoItem>,
}

#[derive(Serialize)]
struct ToDoItem {
    id: i64,
    item: String,
}

#[derive(Serialize)]
pub struct StatusMessage {
    message: String,
}

#[get("/todo")]
pub fn fetch_all_todo_items() -> Result<Json<ToDoList>, String> {
    let db_connection = match Connection::open("db.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err("Fail to connect to database".into());
        }
    };

    let mut statment = match db_connection.prepare("select * from todo_list") {
        Ok(statiment) => statiment,
        Err(_) => return Err("Fail to preper quere".into()),
    };

    let results = statment.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(ToDoItem {
            id: row.get(0)?,
            item: row.get(1)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(ToDoList { items })),
                Err(_) => Err("Could not colletct Items".into()),
            }
        }
        Err(_) => Err("Fail to fetch todo items".into()),
    }
}

#[post("/todo", format = "json", data = "<item>")]
pub fn add_todo_item(item: Json<String>) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("db.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err("Fail to connect to database".into());
        }
    };

    let mut statment =
        match db_connection.prepare("insert into  todo_list (id, item) values (null, $1);") {
            Ok(statiment) => statiment,
            Err(_) => return Err("Fail to preper quere".into()),
        };

    let results = statment.execute(&[&item.0]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rowns inserted", rows_affected),
        })),
        Err(_) => Err("Fail to insert todo items".into()),
    }
}

#[delete("/todo/<id>")]
pub fn remove_todo_item(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("db.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err("Fail to connect to database".into());
        }
    };

    let mut statment = match db_connection.prepare("delete from todo_list where id = $1;") {
        Ok(statiment) => statiment,
        Err(_) => return Err("Fail to preper quere".into()),
    };

    let results = statment.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rowns deleted", rows_affected),
        })),
        Err(_) => Err("Fail to insert todo items".into()),
    }
}
