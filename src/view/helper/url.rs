use handlebars::{Handlebars, Helper, RenderError, RenderContext};
use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn gravatar(helper: &Helper, _: &Handlebars, context: &mut RenderContext) -> Result<(), RenderError>
{
    let hash = {
        if let Some(gravatar_email) = helper.param(0).unwrap().value().as_str() {
            let mut hasher = Md5::new();
            hasher.input_str(&gravatar_email.trim().to_lowercase());
            hasher.result_str()
        } else {
            "".to_string()
        }
    };

    let url = format!("//gravatar.com/avatar/{}?d=mm&s={}", hash, 32);
    context.writer.write(url.as_bytes().as_ref())?;
    Ok(())
}