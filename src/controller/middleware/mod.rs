pub mod csrf;

use url::form_urlencoded;
use futures::future;
use hyper::server::Response;
use hyper::Body;

use std::borrow::Cow;

use controller::router::BoxFutureResponse;
use controller::context::RequestContext;
use self::csrf::csrf_protection;

pub fn around<F>(mut request: RequestContext, body: Body, seed: F) -> BoxFutureResponse
    where F: Fn(RequestContext) -> Response + 'static
{
    use futures::Stream;
    use futures::Future;

    Box::new(body
        .concat2()
        .and_then(|body_data| {
            let vec: Vec<u8> = body_data.iter().cloned().collect();
            let body_parsed = form_urlencoded::parse(vec.as_ref())
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect();

            request.body = Some(body_parsed);

            future::ok(csrf_protection(request, seed))
        })
    )
}
