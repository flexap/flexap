use super::base;
use config::Config;
use controller::context::{RequestContext, ResponseContext};
use controller::dto::LoginFormDto;
use model::service::DbService;

pub fn home(request: RequestContext) -> ResponseContext
{
    base::db_user(&request, |_request, _user, db_list| {
        base::redirect(&format!("/db/{}", db_list[0]))
    })
}

pub fn login(request: RequestContext) -> ResponseContext
{
    if request.user.is_some() {
        return base::main_redirect_response()
    }

    let mut errors: Vec<String> = vec![];

    if request.is_post() && request.body.is_some() {
        let user = LoginFormDto::from(request.body.as_ref().unwrap()).user();
        match DbService::new(&user) {
            Ok(db) =>  match db.list() {
                Ok(list) => {
                    let mut response = if list.len() > 0 {
                        base::redirect( & format ! ("/db/{}", list[0]))
                    } else {
                        base::main_redirect_response()
                    };
                    response.user = Some(user);
                    return response;
                },
                Err(error) => errors.push(format!("{}", error))
            },
            Err(error) => errors.push(format!("{}", error))
        }
    }

    base::render(&request, |_request, replacements| {
        let title = format!("{} - {}", Config::idem().app_name, "Sign In");
        replacements.insert("title", title);
        replacements.insert("form_name", LoginFormDto::form_name());
        replacements.insert("errors", &errors);

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