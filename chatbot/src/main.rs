use actix_cors::Cors;
// use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{middleware, web, App, HttpServer};

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "info,actix_web=info");

    #[cfg(not(feature = "mongo"))]
    let data = web::Data::new(rustserver::connection_mysql::init_pool());
    #[cfg(feature = "mongo")]
    let data = web::Data::new(rustserver::connection_mongo::init_mongo().await);

    // start http server
    HttpServer::new(move || {
        let cors = Cors::default().max_age(3600).allowed_methods(vec!["GET", "POST"]);
        // let store = MemoryStore::new();
        // .allowed_origin("*")
        // .allowed_methods(vec!["GET", "POST"])
        // .max_age(3600);
        App::new()
            .wrap(cors)
            // Limitation: 70 Requests / Second
            // .wrap(
            //     RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
            //         .with_interval(Duration::from_secs(60))
            //         .with_max_requests(70),
            // )
            .app_data(data.clone()) // <- store db pool in app state
            .wrap(middleware::Logger::default())
            // .service(rustserver::test::hello)
            // .service(rustserver::test::db_test)
            .service(rustserver::route::get_notices)
            .service(rustserver::notice::get_today_notice)
            .service(rustserver::notice::get_more_today_notice)
            .service(rustserver::notice::get_yesterday_notice)
            .service(rustserver::notice::get_last_notice)
            .service(rustserver::notice::get_keyword_notice)
            .service(rustserver::notice::get_category)
            .service(rustserver::notice::get_category_notice)
            .service(rustserver::info::get_weather)
            .service(rustserver::info::get_schedule)
            .service(rustserver::info::get_library)
            .service(rustserver::info::get_people)
            .service(rustserver::info::get_map)
    })
    .bind(rustserver::SERVER)?
    .run()
    .await
}
