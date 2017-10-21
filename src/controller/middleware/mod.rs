pub mod csrf;

use url::form_urlencoded;
use futures::future;
use hyper::server::Response;
use hyper::Body;
use hyper::Chunk;
use hyper::Headers;
use hyper::header::ContentType;
use mime;

use controller::router::BoxFutureResponse;
use controller::context::RequestContext;
use controller::context::BodyContent;
use self::csrf::csrf_protection;

pub fn around<F>(mut request: RequestContext, body: Body, seed: F) -> BoxFutureResponse
    where F: Fn(RequestContext) -> Response + 'static
{
    use futures::Stream;
    use futures::Future;

    Box::new(body
        .concat2()
        .and_then(|body_data| {
            request.body = parse_body(&request.headers, body_data);

            future::ok(csrf_protection(request, seed))
        })
    )
}

fn parse_body(headers: &Headers, body_data: Chunk) -> Option<BodyContent>
{
    if body_data.is_empty() {
        None
    } else {
        let is_multipart_form_data = if let Some(&ContentType(ref content_mime)) = headers.get::<ContentType>() {
            content_mime.type_() == mime::MULTIPART && content_mime.subtype() == mime::FORM_DATA
        } else {
            false
        };

        let body_parsed = if is_multipart_form_data {
            parse_multipart_form_data(headers, body_data)
        } else {
            parse_form_urlencoded(body_data)
        };

        Some(body_parsed)
    }
}

fn parse_form_urlencoded(body_data: Chunk) -> BodyContent
{
    let vec: Vec<u8> = body_data.iter().cloned().collect();
    form_urlencoded::parse(vec.as_ref())
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

// TODO: realize this function
// https://github.com/abonander/multipart-async
// https://ru.wikipedia.org/wiki/HTTP#Множественное_содержимое
fn parse_multipart_form_data(headers: &Headers, body_data: Chunk) -> BodyContent
{
    if let Some(boundary) = get_boundary(headers) {
        println!("multipart request received, boundary: {}", boundary);
    }
    parse_form_urlencoded(body_data) // TODO: this is stub
}

fn get_boundary(headers: &Headers) -> Option<String> {
    headers.get::<ContentType>()
        .and_then(|&ContentType(ref mime)| {
            mime.get_param(mime::BOUNDARY).map(|n|n.as_ref().into())
        })
}
