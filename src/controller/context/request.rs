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
    pub uri_path_chunks: Vec<String>,
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
        let uri_path_chunks = uri.path()[1..]
            .split('/')
            .map(|chunk| {
                match percent_decode(chunk.as_bytes()).decode_utf8() {
                    Ok(decoded) => decoded.to_string(),
                    Err(_) => chunk.to_string()
                }
            })
            .collect();

        (RequestContext {
            uri,
            uri_path_chunks,
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
}
