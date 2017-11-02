use hyper::server::Response;
use hyper::header::{ContentLength, ContentType, Header, Location};
use hyper::StatusCode;
use mime;

use std::error::Error;

use view::renderer::{Renderer, TemplateReplacements};
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
    where A: Fn(&RequestContext, &User, Option<&String>, Vec<String>) -> ResponseContext
{
    if let Some(user) = request.user.as_ref() {
        match || {
            let db = DbService::new(user)?;

            let list = db.list()?;
            if list.len() > 0 {
                let db_name = if request.uri_path_chunks.len() > 1 {
                    Some(&request.uri_path_chunks[1])
                } else {
                    None
                };

                if db_name.is_some() && !list.contains(db_name.as_ref().unwrap()) {
                    Err(From::from(format!("DB with name '{}' is not allowed", db_name.as_ref().unwrap())))
                } else {
                    Ok(action(request, user, db_name, list))
                }
            } else {
                Err(From::from("db list is empty"))
            }
        } as Result<_, Box<Error>>() {
            Ok(response) => response,
            Err(error) => {
                println!("ERROR base::db_user - for user \"{}\" produce error: {}", user.name, error);
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
    where A: Fn(&RequestContext, &mut TemplateReplacements) -> Result<String, Box<Error>>
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

pub fn replacements(request: &RequestContext) -> Result<TemplateReplacements, Box<Error>>
{
    let mut replacements = TemplateReplacements::new();
    replacements.insert("brand", &Config::idem().app_name);
    replacements.insert("title", &Config::idem().app_name);

    if request.uri_path_chunks.len() > 0 {
        replacements.insert("section", &request.uri_path_chunks[0]);
    }

    let ref lang = "en";
    replacements.insert("lang", lang);

    if let Some(ref token) = request.csrf_token {
        replacements.insert("csrf_token", token);
        //    replacements.insert("csrf_url_token", &url_token);
    }

    if let Some(ref user) = request.user {
        replacements.insert("user", user);
    }

    Ok(replacements)
}
