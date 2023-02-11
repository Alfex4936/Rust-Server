#![allow(proc_macro_derive_resolution_fallback)]

use crate::info::*;
use crate::notice::*;
use crate::route::{dummy, get_notices};
use actix_web::dev::HttpServiceFactory;
use actix_web::http::Method;
use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::Future;
use std::pin::Pin;

pub enum Function {
    Param(fn(actix_web::web::Path<usize>) -> HttpResponse),
    NoParam(fn() -> HttpResponse),
}

// a struct to hold the services
pub struct Service {
    path: &'static str,
    function: Box<dyn Fn(actix_web::web::Path<usize>) -> HttpResponse + 'static>,
    handler: actix_web::http::Method,
}

// Implement a method to return a list of all services
impl Service {
    pub fn all() -> Vec<Service> {
        // let my_functions: Vec<fn() -> Pin<Box<dyn Future<Output = HttpResponse>>>> = vec![
        //     || Box::pin(get_notices),
        //     || Box::pin(get_today_notice),
        //     || Box::pin(get_more_today_notice),
        //     || Box::pin(get_yesterday_notice),
        // ]
        // .iter();

        vec![Service {
            function: Box::new(dummy),
            path: "/notices",
            handler: Method::GET,
        }]
    }
}

// Implement a method to convert a service into an `actix_web::App`
impl Into<&mut ServiceConfig> for Service {
    fn into(self) -> Box<Fn(&mut ServiceConfig)> {
        Box::new(move |cfg: &mut ServiceConfig| {
            cfg.service(
                web::resource(self.path).route(web::method(self.handler).to(*self.function)),
            );
        })
    }
}
