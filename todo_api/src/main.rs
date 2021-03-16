#![feature(proc_macro_hygiene, decl_macro)]

mod todo;
#[macro_use]
extern crate rocket;
use rusqlite::Connection;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    {
        let db_connection = Connection::open("db.sqlite").unwrap();

        db_connection
            .execute(
                "create table if not exists todo_list (
            id integer primary key,
            item varchar(64) not null)",
                rusqlite::NO_PARAMS,
            )
            .unwrap();
    }

    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                todo::fetch_all_todo_items,
                todo::add_todo_item,
                todo::remove_todo_item
            ],
        )
        .launch();
}
