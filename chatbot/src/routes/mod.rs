pub mod info;
pub mod notice;
pub mod user;

pub use crate::db::connection_mongo::DbPool;

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use kakao_rs::components::basics::Template;

#[derive(Serialize)]
pub struct Kakao {
    #[serde(flatten)]
    template: Template,
}

pub use info::init_info;
pub use notice::init_notice;
pub use user::init_user;

// Responder
impl Responder for Kakao {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
