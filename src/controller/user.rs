use hyper::server::Response;

use serde_json::value::to_value as to_json_value;
use std::error::Error;

use super::base;
use config::Config;
use controller::context::RequestContext;
use controller::dto::LoginFormDto;
use model::service::db;

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
    if base::is_logined(&request) {
        return base::main_redirect_response()
    }

    let mut errors: Vec<String> = vec![];

    if request.is_post() && request.body.is_some() {
        let user = LoginFormDto::from(request.body.as_ref().unwrap()).user();
        match db::list(&user) {
            Ok(db_list) => {
                if db_list.len() > 0 {
                    return base::redirect(&format!("/db/{}", db_list[0]));
                } else {
                    return base::main_redirect_response()
                }
            },
            Err(error) => errors.push(format!("{}", error))
        }
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
