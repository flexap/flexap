use handlebars::Handlebars;
use handlebars::RenderError;
use handlebars::RenderContext;
use handlebars::Helper;
use serde_json::Value;

pub fn eq(helper: &Helper, _: &Handlebars, context: &mut RenderContext) -> Result<(), RenderError>
{
    let a = helper.param(0).unwrap().value().as_str().unwrap();
    let b = helper.param(1).unwrap().value().as_str().unwrap();
    let result = a == b;

    context.writer.write(Value::from(result).to_string().into_bytes().as_ref())?;
    Ok(())
}