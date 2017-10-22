use hyper::server::Response;

use serde_json::value::to_value as to_json_value;
use std::error::Error;

use super::base;
use config::Config;
use controller::context::RequestContext;
use controller::dto::LoginFormDto;

pub fn home(request: RequestContext) -> Response
{
    if base::is_logined(&request) {
        base::redirect("/db/")
    } else {
        base::redirect("/user/login")
    }
}

pub fn login(request: RequestContext) -> Response
{
    let mut errors: Vec<String> = vec![];
    let mut is_logined = base::is_logined(&request);

    if !is_logined && request.is_post() && request.body.is_some() {
        let login_form_dto = LoginFormDto::from(request.body.as_ref().unwrap());

        is_logined = true;
    }

    if is_logined {
        return base::main_redirect_response()
    }

    base::render(&request, |_request, replacements| {
        let title = format!("{} - {}", Config::idem().app_name, "Sign In");
        replacements.insert("title".to_owned(), to_json_value(title)?);
        replacements.insert("form_name".to_owned(), to_json_value(LoginFormDto::form_name())?);

        replacements.insert(
            "errors".to_owned(),
            to_json_value(&errors)?
        );

        Ok("user/login".to_owned())
    })
}
