use url::form_urlencoded;
use hyper::Chunk;
use mime;

use controller::context::BodyContent;

pub fn parse_body(content_type: Option<&mime::Mime>, body_data: Chunk) -> Option<BodyContent>
{
    if body_data.is_empty() {
        None
    } else {
        let is_multipart_form_data = if let Some(ref content_mime) = content_type {
            content_mime.type_() == mime::MULTIPART && content_mime.subtype() == mime::FORM_DATA
        } else {
            false
        };

        let body_parsed = if is_multipart_form_data {
            parse_multipart_form_data(content_type, body_data)
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
fn parse_multipart_form_data(content_type: Option<&mime::Mime>, body_data: Chunk) -> BodyContent
{
    if let Some(boundary) = get_boundary(content_type) {
        println!("multipart request received, boundary: {}", boundary);
    }
    parse_form_urlencoded(body_data) // TODO: this is stub
}

fn get_boundary(content_type: Option<&mime::Mime>) -> Option<String> {
    content_type
        .and_then(|content_mime| {
            content_mime.get_param(mime::BOUNDARY).map(|boundary|boundary.as_ref().into())
        })
}
