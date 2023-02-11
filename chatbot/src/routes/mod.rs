pub mod info;
pub mod notice;
// pub mod route;

#[cfg(feature = "mongo")]
pub use crate::db::connection_mongo::DbPool;
#[cfg(not(feature = "mongo"))]
pub use crate::db::connection_mysql::DbPool;

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use kakao_rs::components::basics::Template;

#[derive(Serialize)]
pub struct Kakao {
    #[serde(flatten)]
    template: Template,
}

pub use info::init_info;
pub use notice::init_notice;

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
