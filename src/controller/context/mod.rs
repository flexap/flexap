use hyper::Request;
use hyper::Body;
use hyper::header::Headers;
use hyper::Method;
use hyper::Uri;

use std::collections::HashMap;

pub struct RequestContext
{
    pub uri: Uri,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<HashMap<String, String>>,
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
}