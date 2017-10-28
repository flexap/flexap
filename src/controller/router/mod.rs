pub mod route;

use futures::Future;
use futures::future;
use hyper::Error;
use hyper::header::ContentLength;
use hyper::server::{Service, Request, Response};
use hyper::StatusCode;
use hyper::Method;

use controller::context::RequestContext;
use controller::middleware::around;
use controller::db;
use controller::user;
use self::route::Route;

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
            Method::Get if uri.is_match("/user/logout") => around(request, body, user::logout),
            _ => Box::new(future::ok(
                RouterService::error_handler(StatusCode::NotFound)
            ))
        }
    }
}