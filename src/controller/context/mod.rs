use hyper::{Request, Response, Body, Method, Uri};
use hyper::header::{ContentType, Headers, SetCookie};
use mime::Mime;
use url::percent_encoding::percent_decode;
use jwt::{encode, decode, Header, Algorithm, Validation};

use std::collections::HashMap;

use config::Config;
use model::entity::User;

pub type BodyContent = HashMap<String, String>;

pub struct RequestContext
{
    pub uri: Uri,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<BodyContent>,
    pub csrf_token: Option<String>
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
            csrf_token: None
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
}

pub trait ResponseContext
{
    fn set_to_jwt(&mut self, user: &User) -> bool;
}

impl ResponseContext for Response
{
    fn set_to_jwt(&mut self, user: &User) -> bool
    {
        let header = Header::default();
        let key = Config::idem().security.cookie_key.as_ref();
        let token = encode(&header, &user, key).unwrap_or("".to_string());

        if !token.is_empty() {
            let cookie = format!("jwt={}; Path=/;", token);
            let headers = self.headers_mut();

            let need_set_cookie_opt = match headers.get_mut() {
                Some(&mut SetCookie(ref mut content)) => {
                    content.push(cookie);
                    None
                },
                _ => Some(SetCookie(vec![cookie]))
            };
            if let Some(set_cookie) = need_set_cookie_opt {
                headers.set(set_cookie);
            }

            true
        } else {
            false
        }
    }
}