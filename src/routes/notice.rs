use rocket::response::content::Json;

#[get("/hello")]
pub fn hello() -> Json<&'static str> {
    Json(
        "{
    'status': 'success',
    'message': 'Hello API!'
}",
    )
}
