use crate::db::connection::Conn;
use crate::db::models::Schedule;
use crate::db::query;
use diesel;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Notice {
    id: u64,
    title: String,
    date: String,
    link: String,
    writer: String,
}

#[get("/hello")]
pub fn hello() -> Json<Notice> {
    let notice = Notice {
        id: 12345,
        title: "공지1".to_string(),
        date: "2021-07-09".to_string(),
        link: "https://".to_string(),
        writer: "CSW".to_string(),
    };
    Json(notice)
}

#[get("/db")]
pub fn db_test(conn: Conn) -> Result<Json<Vec<Schedule>>, Status> {
    query::show_scheds(&conn)
        .map(|sched| Json(sched))
        .map_err(|error| error_status(error))

    // for row in result {
    //     let (id, content, start_date, end_date): (i32, String, String, String) =
    //         mysql::from_row(row.unwrap());
    //     println!("id: {}, content: {}", id, content);
    // }

    // let notice = Notice {
    //     id: 12345,
    //     title: "공지1".to_string(),
    //     date: "2021-07-09".to_string(),
    //     link: "https://".to_string(),
    //     writer: "CSW".to_string(),
    // };
    // Json(notice)
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
