use serde_json::value::to_value as to_json_value;

use super::base;
use config::Config;
use controller::context::{RequestContext, ResponseContext};
use controller::dto::LoginFormDto;
use model::service::db;

pub fn home(request: RequestContext) -> ResponseContext
{
    if let Some(_user) = request.user.as_ref() {
        base::redirect("/db/")
    } else {
        base::redirect("/user/login")
    }
}

pub fn login(request: RequestContext) -> ResponseContext
{
    if request.user.is_some() {
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
                response.user = Some(user);
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

pub fn logout(request: RequestContext) -> ResponseContext
{
    let mut response = base::main_redirect_response();
    if request.user.is_some() {
        response.clean_user = true;
    }
    response
}