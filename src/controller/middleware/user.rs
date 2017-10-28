use hyper::header::Cookie;
use jwt::{encode, decode, Header, Validation};

use config::Config;
use controller::context::{RequestContext, ResponseContext};
use model::entity::User;

pub fn user_session<F>(mut request: RequestContext, seed: F) -> ResponseContext
    where F: Fn(RequestContext) -> ResponseContext
{
    if let Some(ref cookie) = request.headers.get::<Cookie>() {
        let token = cookie.get("jwt").unwrap_or("");
        if !token.is_empty() {
            let key = Config::idem().security.cookie_key.as_ref();

            match decode::<User>(token, key, &Validation::default()) {
                Ok(token_data) => {
                    request.user = Some(token_data.claims);
                },
                Err(error) => println!("Error jwt decode: {:?}", error)
            }
        }
    }

    let mut response = seed(request);

    if response.clean_user {
        response.set_cookie("jwt=deleted; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT".to_string());
    } else if response.user.is_some() {
        let header = Header::default();
        let key = Config::idem().security.cookie_key.as_ref();
        let token = encode(&header, response.user.as_ref().unwrap(), key).unwrap_or("".to_string());

        if !token.is_empty() {
            response.set_cookie(format!("jwt={}; Path=/;", token));
        }
    }
    response
}