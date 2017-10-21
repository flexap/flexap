use serde_json::value::Value;
use handlebars::JsonRender;
use handlebars::to_json;
use serde_json::value::Map;

use hyper::server::Response;
use hyper::header::Header;
use hyper::header::Location;
use hyper::header::ContentLength;
use hyper::header::ContentType;
use hyper::StatusCode;
use mime;

use std::error::Error;

use view::renderer::Renderer;
use config::Config;
use controller::context::RequestContext;

pub fn redirect(url: &str) -> Response
{
    let raw_url = url.into();
    Response::new()
        .with_status(StatusCode::Found)
        .with_header(Location::parse_header(&raw_url).unwrap_or(Location::new("/")))
}

pub fn main_redirect_response() -> Response
{
    redirect("/")
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

pub fn error_response(msg: String) -> Response
{
    let response = Response::new()
        .with_header(ContentLength(msg.len() as u64))
        .with_body(msg);
    response.with_status(StatusCode::InternalServerError)
}

#[allow(dead_code)]
pub fn render<A>(request: &RequestContext, action: A) -> Response
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
    response.with_body(result)
}

pub fn replacements(request: &RequestContext) -> Result<Map<String, Value>, Box<Error>>
{
    let mut replacements = Map::new();
    replacements.insert("brand".to_string(), to_json(&Config::idem().app_name));
    replacements.insert("title".to_string(), to_json(&Config::idem().app_name));

    let ref lang = "en";
    replacements.insert("lang".to_owned(), to_json(lang));

    if let Some(ref token) = request.csrf_token {
        println!("New csrf token: {:?}", token);
        replacements.insert("csrf_token".to_owned(), to_json(token));
        //    replacements.insert("csrf_url_token".to_owned(), to_json(&url_token));
    }

    Ok(replacements)
}
