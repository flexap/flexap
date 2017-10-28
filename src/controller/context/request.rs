use hyper::{Request, Body, Method, Uri};
use hyper::header::{ContentType, Headers};
use mime::Mime;
use url::percent_encoding::percent_decode;

use std::collections::HashMap;

use model::entity::User;

pub type BodyContent = HashMap<String, String>;

pub struct RequestContext
{
    pub uri: Uri,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<BodyContent>,
    pub csrf_token: Option<String>,
    pub user: Option<User>
}

impl RequestContext
{
    pub fn from_request(request: Request) -> (Self, Body)
    {
        let (method, uri, _version, headers, body) = request.deconstruct();

        (RequestContext {
            uri,
            method,
            headers,
            body: None,
            csrf_token: None,
            user: None
        }, body)
    }

    pub fn content_type(&self) -> Option<&Mime>
    {
        if let Some(&ContentType(ref mime)) = self.headers.get() {
            Some(mime)
        } else {
            None
        }
    }

    pub fn is_post(&self) -> bool
    {
        self.method == Method::Post
    }

    pub fn uri_path_chunks(&self) -> Vec<String>
    {
        self.uri.path()[1..]
            .split('/')
            .map(|name| {
                match percent_decode(name.as_bytes()).decode_utf8() {
                    Ok(decoded) => decoded.to_string(),
                    Err(_) => name.to_string()
                }
            })
            .collect()
    }

//    pub fn user(&mut self) -> Option<&User>
//    {
//        if self.user.is_some() {
//            return self.user.as_ref();
//        }
//
//        if let Some(ref cookie) = self.headers.get::<Cookie>() {
//            let token = cookie.get("jwt").unwrap_or("");
//            if !token.is_empty() {
//                let key = Config::idem().security.cookie_key.as_ref();
//
//                match decode::<User>(token, key, &Validation::default()) {
//                    Ok(token_data) => {
//                        self.user = Some(token_data.claims);
//                        return self.user.as_ref();
//                    },
//                    Err(error) => println!("Error jwt decode: {:?}", error)
//                }
//            }
//        }
//        None
//    }
}
