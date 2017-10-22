pub mod body;
pub mod csrf;

use futures::future;
use hyper::server::Response;
use hyper::Body;

use controller::router::BoxFutureResponse;
use controller::context::RequestContext;
use self::body::parse_body;
use self::csrf::csrf_protection;

pub fn around<F>(mut request: RequestContext, body: Body, seed: F) -> BoxFutureResponse
    where F: Fn(RequestContext) -> Response + 'static
{
    use futures::Stream;
    use futures::Future;

    Box::new(body
        .concat2()
        .and_then(|body_data| {
            request.body = parse_body(request.content_type(), body_data);

            future::ok(csrf_protection(request, seed))
        })
    )
}
