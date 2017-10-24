use hyper::server::Response;
use serde_json::value::to_value as to_json_value;

use std::error::Error;

use super::base;
use config::Config;
use controller::context::{RequestContext, ResponseContext};
use controller::dto::LoginFormDto;
use model::service::db;

pub fn home(request: RequestContext) -> Response
{
    if let Some(_user) = request.user() {
        base::redirect("/db/")
    } else {
        base::redirect("/user/login")
    }
}

pub fn login(request: RequestContext) -> Response
{
    if request.user().is_some() {
        return base::main_redirect_response()
    }

    let mut errors: Vec<String> = vec![];

    if request.is_post() && request.body.is_some() {
        let user = LoginFormDto::from(request.body.as_ref().unwrap()).user();
        match db::list(&user) {
            Ok(db_list) => {
                let mut response = if db_list.len() > 0 {
                    base::redirect(&format!("/db/{}", db_list[0]))
                } else {
                    base::main_redirect_response()
                };
                response.set_user(&user);
                return response;
            },
            Err(error) => errors.push(format!("{}", error))
        }
    }

    base::render(&request, |_request, replacements| {
        let title = format!("{} - {}", Config::idem().app_name, "Sign In");
        replacements.insert("title".to_string(), to_json_value(title)?);
        replacements.insert("form_name".to_string(), to_json_value(LoginFormDto::form_name())?);

        replacements.insert(
            "errors".to_string(),
            to_json_value(&errors)?
        );

        Ok("user/login".to_owned())
    })
}
