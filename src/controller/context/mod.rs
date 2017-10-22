use hyper::Request;
use hyper::Body;
use hyper::header::ContentType;
use hyper::header::Headers;
use hyper::Method;
use hyper::Uri;
use mime::Mime;

use std::collections::HashMap;

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
}