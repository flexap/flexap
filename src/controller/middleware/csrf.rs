use csrf::{AesGcmCsrfProtection, CsrfProtection, CSRF_HEADER};
//use data_encoding::BASE64;

use config::Config;
use controller::context::{RequestContext, ResponseContext};
use controller::middleware::user::user_session;

use std::str;

header! { (XCsrfHeader, CSRF_HEADER) => [String] }

pub fn csrf_protection<F>(mut request: RequestContext, seed: F) -> ResponseContext
    where F: Fn(RequestContext) -> ResponseContext
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

    let response = user_session(request, seed);
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