use serde_json::value::{Map, Value};
use handlebars::to_json;

use hyper::server::Response;
use hyper::header::{ContentLength, ContentType, Header, Location};
use hyper::StatusCode;
use mime;

use std::error::Error;

use view::renderer::Renderer;
use config::Config;
use controller::context::{RequestContext, ResponseContext};
use model::entity::User;
use model::service::DbService;

pub fn redirect(url: &str) -> ResponseContext
{
    let raw_url = url.into();
    ResponseContext::from_response(
        Response::new()
            .with_status(StatusCode::Found)
            .with_header(Location::parse_header(&raw_url).unwrap_or(Location::new("/")))
    )
}

pub fn main_redirect_response() -> ResponseContext
{
    redirect("/")
}

pub fn db_user<A>(request: &RequestContext, action: A) -> ResponseContext
    where A: Fn(&RequestContext, &User, Vec<String>) -> ResponseContext
{
    if let Some(user) = request.user.as_ref() {
        let db = DbService::new(user);

        match db.list() {
            Ok(list) => {
                if list.len() > 0 {
                    action(request, user, list)
                } else {
                    println!("ERROR base::db_user - db.list for user \"{}\" is empty", user.name);
                    redirect("/error")
                }
            },
            Err(error) => {
                println!("ERROR base::db_user - {}", error);
                redirect("/error")
            }
        }
    } else {
        redirect("/user/login")
    }
}

//pub fn get_tail_param(request: &Request, validator: Validator) -> Option<String>
//{
//    if let Some(param) = request.path().rsplit('/').next() {
//        if validator.validate(param).is_ok() {
//            return Some(param.to_string());
//        }
//    }
//    None
//}

//pub fn error_response(msg: String) -> Response
//{
//    let response = Response::new()
//        .with_header(ContentLength(msg.len() as u64))
//        .with_body(msg);
//    response.with_status(StatusCode::InternalServerError)
//}

pub fn render<A>(request: &RequestContext, action: A) -> ResponseContext
    where A: Fn(&RequestContext, &mut Map<String, Value>) -> Result<String, Box<Error>>
{
    let mut response = Response::new();
    let result = match || {
        let mut replacements = replacements(request)?;
        let view = action(&request, &mut replacements)?;
        let rendered = Renderer::idem().render(&view, &replacements)?;
        Ok(rendered)
    } as Result<_, Box<Error>>() {
        Ok(v) => {
            response.headers_mut().set(ContentType(mime::TEXT_HTML));
            v
        },
        Err(e) => {
            response.headers_mut().set(ContentType(mime::TEXT_PLAIN));
            format!("Error: {:?}", e)
        }
    };
    let result = result.as_bytes().to_vec();
    response.headers_mut().set(ContentLength(result.len() as u64));
    ResponseContext::from_response(
        response.with_body(result)
    )
}

pub fn replacements(request: &RequestContext) -> Result<Map<String, Value>, Box<Error>>
{
    let mut replacements = Map::new();
    replacements.insert("brand".to_string(), to_json(&Config::idem().app_name));
    replacements.insert("title".to_string(), to_json(&Config::idem().app_name));

    if request.uri_path_chunks.len() > 0 {
        replacements.insert("section".to_string(), to_json(&request.uri_path_chunks[0]));
    }

    let ref lang = "en";
    replacements.insert("lang".to_owned(), to_json(lang));

    if let Some(ref token) = request.csrf_token {
        replacements.insert("csrf_token".to_owned(), to_json(token));
        //    replacements.insert("csrf_url_token".to_owned(), to_json(&url_token));
    }

    if let Some(ref user) = request.user {
        replacements.insert("user".to_owned(), to_json(user));
    }

    Ok(replacements)
}
