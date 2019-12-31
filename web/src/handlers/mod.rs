use actix_web::http::StatusCode;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub task: String,
}

pub fn todos(_req: HttpRequest) -> Result<HttpResponse> {
    let conn = db::establish_connection();
    let tasks = db::Todo::todos(&conn);

    Ok(HttpResponse::build(StatusCode::OK).json(tasks))
}

pub fn todo(_req: HttpRequest, task_id: Path<(i32,)>) -> Result<HttpResponse> {
    let conn = db::establish_connection();
    let task = db::Todo::todo(&conn, task_id.0);

    Ok(HttpResponse::build(StatusCode::OK).json(task))
}

pub fn create(_req: HttpRequest, obj: Json<Todo>) -> Result<HttpResponse> {
    let conn = db::establish_connection();
    db::Todo::create(&conn, &obj.task);

    Ok(HttpResponse::new(StatusCode::CREATED))
}

pub fn update(_req: HttpRequest, task_id: Path<(i32,)>) -> Result<HttpResponse> {
    let conn = db::establish_connection();
    db::Todo::update(&conn, task_id.0);

    Ok(HttpResponse::new(StatusCode::CREATED))
}

pub fn delete(_req: HttpRequest, task_id: Path<(i32,)>) -> Result<HttpResponse> {
    let conn = db::establish_connection();
    db::Todo::delete(&conn, task_id.0);

    Ok(HttpResponse::new(StatusCode::ACCEPTED))
}
