use hyper::server::Response;
use csrf::CSRF_HEADER;
use csrf::AesGcmCsrfProtection;
use csrf::CsrfProtection;
//use data_encoding::BASE64;

use config::Config;
use controller::context::RequestContext;

use std::str;

header! { (XCsrfHeader, CSRF_HEADER) => [String] }

pub fn csrf_protection<F>(mut request: RequestContext, seed: F) -> Response
    where F: Fn(RequestContext) -> Response
{

    if let Some(csrf_header) = request.headers.get::<XCsrfHeader>() {
        let &XCsrfHeader(ref csrf) = csrf_header;
        println!("Incoming csrf header: {:?}", csrf);
    }

    if let Some(ref body) = request.body {
        println!("Body with csrf: {:?}", body);
    }

    let (token, cookie) = new_token_pair(None);
    request.csrf_token = Some(token);

    let mut response = seed(request);
    response
}

fn new_token_pair(previous_token: Option<&[u8; 64]>) -> (String, String)
{
    let mut key: [u8;32] = Default::default();
    key.copy_from_slice(Config::idem().security.csrf_protection_key.as_bytes());

    let protect = AesGcmCsrfProtection::from_key(key);
    let (token, cookie) = protect.generate_token_pair(previous_token, Config::idem().security.csrf_token_ttl_seconds)
        .expect("couldn't generate token/cookie pair");

    (token.b64_string(), cookie.b64_string())
}