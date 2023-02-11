#![allow(proc_macro_derive_resolution_fallback)]

use actix_cors::Cors;
use std::time::Duration;
// use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{middleware, web, App, HttpServer};

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set the logging level for the actix_web crate to info
    //std::env::set_var("RUST_LOG", "info,actix_web=info");

    // Log the type of the get_notices function, which returns a reference to a string
    // print_type_of(&rustserver::route::get_notices); // &str

    // Initialize the MongoDB connection
    let data = web::Data::new(rustserver::connection_mongo::init_mongo().await);

    // Start the HTTP server
    HttpServer::new(move || {
        // Set up CORS with a maximum age of 3600 seconds, allowing only GET and POST methods
        let cors = Cors::default()
            .max_age(3600)
            .allowed_methods(vec!["GET", "POST"]);

        App::new()
            .wrap(cors)
            .app_data(data.clone()) // clone the MongoDB connection data for each HTTP request
            .wrap(middleware::Logger::default()) // Use the default logger for request/response logs
            .service(
                // Mount the notice routes under the "/notice" path
                web::scope("/notice").configure(rustserver::notice::init_notice),
            )
            .service(
                // Mount the info routes under the "/info" path
                web::scope("/info").configure(rustserver::info::init_info),
            )
    })
    .keep_alive(Duration::from_secs(10)) // Keep the connection alive for 10 seconds
    .bind(rustserver::SERVER)? // Bind to the server address specified in the rustserver module
    .run() // Start the HTTP server
    .await
}
