use handlebars::{Handlebars, Helper, RenderError, RenderContext};

pub fn eq(helper: &Helper, _: &Handlebars, context: &mut RenderContext) -> Result<(), RenderError>
{
    let a = helper.param(0).unwrap().value().as_str().unwrap();
    let b = helper.param(1).unwrap().value().as_str().unwrap();
    if a == b {
        context.writer.write("true".as_bytes().as_ref())?;
    }
    Ok(())
}