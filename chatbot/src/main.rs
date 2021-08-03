use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};
use rustserver;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=info");
    // start http server
    HttpServer::new(|| {
        let cors = Cors::permissive();
        // .allowed_origin("*")
        // .allowed_methods(vec!["GET", "POST"])
        // .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(rustserver::connection::init_pool().clone())) // <- store db pool in app state
            .wrap(middleware::Logger::default())
            .service(rustserver::test::hello)
            .service(rustserver::test::db_test)
            .service(rustserver::test::get_notices)
            .service(rustserver::chatbot::ask_weather)
            .service(rustserver::chatbot::get_today_notice)
            .service(rustserver::chatbot::get_schedule)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
