use hyper::header::Cookie;
use csrf::{AesGcmCsrfProtection, CsrfProtection, CSRF_HEADER, CSRF_COOKIE_NAME};
use data_encoding::base64;

use config::Config;
use controller::base;
use controller::context::{RequestContext, ResponseContext};
use controller::middleware::user::user_session;

use std::str;

header! { (XCsrfHeader, CSRF_HEADER) => [String] }

pub fn csrf_protection<F>(mut request: RequestContext, seed: F) -> ResponseContext
    where F: Fn(RequestContext) -> ResponseContext
{
    let protect = make_protect();

    if request.is_post() {
        let (token_bytes, cookie_bytes) = parse_token_pair(&request);
        if token_bytes.is_empty() || cookie_bytes.is_empty() {
            println!("ERROR csrf_protection - empty token pair");
            return base::redirect("/error");
        }

        if !verify_token_pair(&protect, &token_bytes, &cookie_bytes) {
            println!("ERROR csrf_protection - token pair is not verified");
            return base::redirect("/error");
        }
    }

    let (token, cookie) = new_token_pair(&protect, None);
    request.csrf_token = Some(token);

    let mut response = user_session(request, seed);
    response.set_cookie(format!("{}={}; Path=/; HttpOnly", CSRF_COOKIE_NAME, cookie));
    response
}

fn make_protect() -> AesGcmCsrfProtection
{
    let mut key: [u8;32] = Default::default();
    key.copy_from_slice(Config::idem().security.csrf_protection_key.as_bytes());

    AesGcmCsrfProtection::from_key(key)
}

fn parse_token_pair(request: &RequestContext) -> (Vec<u8>, Vec<u8>)
{
    let token = if let Some(ref body) = request.body {
        let token = body.get("csrf-token").map(AsRef::as_ref).unwrap_or("");
        println!("Incoming csrf body token: {:?}", token);
        token
    } else if let Some(csrf_header) = request.headers.get::<XCsrfHeader>() {
        // TODO: implement header csrf validation
        let &XCsrfHeader(ref csrf) = csrf_header;
        println!("Incoming csrf header: {:?}", csrf);
        ""
    } else {
        ""
    };

    let cookie = if let Some(ref cookie) = request.headers.get::<Cookie>() {
        let token = cookie.get(CSRF_COOKIE_NAME).unwrap_or("");
        println!("Incoming csrf cookie token: {:?}", token);
        token
    } else {
        ""
    };

    (
        base64::decode(token.as_bytes()).unwrap_or(vec![]),
        base64::decode(cookie.as_bytes()).unwrap_or(vec![])
    )
}

fn verify_token_pair<T>(protect: &T, token_bytes: &[u8], cookie_bytes: &[u8]) -> bool
    where T: CsrfProtection
{
    match protect.parse_token(token_bytes) {
        Ok(parsed_token) => match protect.parse_cookie(cookie_bytes) {
            Ok(parsed_cookie) => {
                protect.verify_token_pair(&parsed_token, &parsed_cookie)
            },
            Err(err) => {
                println!("ERROR verify_token_pair - {:?}", err);
                false
            }
        },
        Err(err) => {
            println!("ERROR verify_token_pair - {:?}", err);
            false
        }
    }
}

fn new_token_pair<T>(protect: &T, previous_token: Option<&[u8; 64]>) -> (String, String)
    where T: CsrfProtection
{
    let ttl = Config::idem().security.csrf_token_ttl_seconds;
    let (token, cookie) = protect.generate_token_pair(previous_token, ttl)
        .expect("couldn't generate token/cookie pair");

    (token.b64_string(), cookie.b64_string())
}