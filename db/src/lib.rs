#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use schema::todos;
use std::env;

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub completed: bool,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub task: &'a str,
    pub completed: bool,
    pub timestamp: chrono::NaiveDateTime,
}

// #[derive(Serialize, Deserialize, Debug, AsChangeset)]
// #[table_name = "todos"]
// pub struct UpdateTodo {
//     pub task: Option<String>,
//     pub completed: Option<bool>,
// }

impl Todo {
    pub fn todos(conn: &SqliteConnection) -> Vec<Todo> {
        use schema::todos::dsl::*;

        todos.load::<Todo>(conn).expect("Error loading todos")
    }

    pub fn todo(conn: &SqliteConnection, task_id: i32) -> Vec<Todo> {
        use schema::todos::dsl::*;

        todos
            .find(task_id)
            .load::<Todo>(conn)
            .expect("Error loading todo")
    }

    pub fn create(conn: &SqliteConnection, task: &str) -> usize {
        let todo = NewTodo {
            task,
            completed: false,
            timestamp: chrono::Utc::now().naive_local(),
        };

        diesel::insert_into(schema::todos::table)
            .values(&todo)
            .execute(conn)
            .expect("Error saving new todo")
    }

    pub fn update(conn: &SqliteConnection, task_id: i32) -> usize {
        use schema::todos::dsl::*;

        diesel::update(todos.find(task_id))
            .set(completed.eq(true))
            .execute(conn)
            .expect("Error updating todo")
    }

    pub fn delete(conn: &SqliteConnection, task_id: i32) -> usize {
        use schema::todos::dsl::*;

        diesel::delete(todos.find(task_id))
            .execute(conn)
            .expect("Error deleting todo")
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("The database url needs to be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
