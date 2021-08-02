use actix_web::{middleware, web, App, HttpServer};
use rustserver;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // start http server
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(rustserver::connection::init_pool().clone())) // <- store db pool in app state
            .wrap(middleware::Logger::default())
            .service(rustserver::test::hello)
            .service(rustserver::test::db_test)
            .service(rustserver::test::get_notices)
            .service(rustserver::chatbot::get_today_notices)
            .service(rustserver::chatbot::ask_weather)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
