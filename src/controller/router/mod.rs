use futures::Future;
use futures::future;
use hyper::Error;
use hyper::header::ContentLength;
use hyper::server::{Service, Request, Response};
use hyper::StatusCode;
use hyper::Method;
use regex::Regex;

use std::collections::HashMap;

use controller::context::RequestContext;
use controller::middleware::around;
use controller::db;
use controller::user;

pub type BoxFutureResponse = Box<Future<Item=Response, Error=Error>>;

#[derive(Debug)]
pub struct RouterService;

impl RouterService
{
    fn error_handler(status_code: StatusCode) -> Response
    {
        let error = "Routing error: page not found";
        let response = Response::new()
            .with_header(ContentLength(error.len() as u64))
            .with_body(error);

        match status_code {
            StatusCode::NotFound => response.with_status(StatusCode::NotFound),
            _ => response.with_status(StatusCode::InternalServerError)
        }
    }
}

lazy_static! {
    static ref REGEX_ROUTES: HashMap<&'static str, Regex> = hashmap! {
        "/db/{name}" => Regex::new("/db/[^/]+/?$").unwrap()
    };
}

pub trait Route
{
    fn is_match(&self, pattern: &str) -> bool;
}

impl Route for String
{
    fn is_match(&self, route: &str) -> bool
    {
        if let Some(regex) = REGEX_ROUTES.get(route) {
            regex.is_match(self.as_str())
        } else {
            self == route
        }
    }
}

impl Service for RouterService
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = BoxFutureResponse;

    fn call(&self, request: Request) -> Self::Future
    {
        let (request, body) = RequestContext::from_request(request);
        let uri = request.uri.path().to_string();

        match request.method.clone() {
            Method::Get if uri.is_match("/") => around(request, body, user::home),
            Method::Get if uri.is_match("/db/{name}") => around(request, body, db::index),
            Method::Get if uri.is_match("/user/login") => around(request, body, user::login),
            Method::Post if uri.is_match("/user/login") => around(request, body, user::login),
            _ => Box::new(future::ok(
                RouterService::error_handler(StatusCode::NotFound)
            ))
        }
    }
}