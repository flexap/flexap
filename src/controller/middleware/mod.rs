pub mod body;
pub mod csrf;
pub mod user;

use futures::future;
use hyper::Body;

use controller::router::BoxFutureResponse;
use controller::context::{RequestContext, ResponseContext};
use self::body::parse_body;
use self::csrf::csrf_protection;

pub fn around<F>(mut request: RequestContext, body: Body, seed: F) -> BoxFutureResponse
    where F: Fn(RequestContext) -> ResponseContext + 'static
{
    use futures::Stream;
    use futures::Future;

    Box::new(body
        .concat2()
        .and_then(|body_data| {
            request.body = parse_body(request.content_type(), body_data);

            let response_context = csrf_protection(request, seed);
            future::ok(response_context.response)
        })
    )
}
