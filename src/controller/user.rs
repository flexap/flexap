use hyper::server::Response;
use hyper::Method;

use serde_json::value::to_value as to_json_value;
use std::error::Error;

use super::base;
use config::Config;
use controller::context::RequestContext;

pub fn home(request: RequestContext) -> Response
{
    base::render(&request, |_request, replacements| {
        Ok("user/login".to_owned())
    })
}

pub fn login(request: RequestContext) -> Response
{
    let mut errors: Vec<String> = vec![];
    let mut is_logined = false;

    if request.method == Method::Post {
    }

    if is_logined {
        return base::main_redirect_response()
    }

    base::render(&request, |_request, replacements| {
        let title = format!("{} - {}", Config::idem().app_name, "Sign In");
        replacements.insert("title".to_owned(), to_json_value(title)?);

        replacements.insert(
            "errors".to_owned(),
            to_json_value(&errors)?
        );

        Ok("user/login".to_owned())
    })
}
