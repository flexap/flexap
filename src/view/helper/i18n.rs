use handlebars::Handlebars;
use handlebars::RenderError;
use handlebars::RenderContext;
use handlebars::Helper;

pub fn translate(helper: &Helper, _: &Handlebars, context: &mut RenderContext) -> Result<(), RenderError>
{
    let messages = hashmap!{
        "Home" => "Home"
    };
    let key = helper.param(0).unwrap().value().as_str().unwrap();
    let value = match messages.get(key) {
        Some(value) => value,
        None => key
    };
    context.writer.write(value.as_bytes().as_ref())?;
    Ok(())
}