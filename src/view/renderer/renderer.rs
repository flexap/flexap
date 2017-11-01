use handlebars::Handlebars;
use super::TemplateSource;

use config::Config;
use view::helper::compare;
use view::helper::i18n;
use view::helper::url;

lazy_static! {
    static ref HANDLEBARS: Handlebars = {
        let mut handlebars = Handlebars::new();
        let source = TemplateSource::new(&Config::idem().view.templates_path, &Config::idem().view.ext);
        source.load(&mut handlebars).unwrap();
        handlebars.register_helper(&Config::idem().view.eq_helper_name, Box::new(compare::eq));
        handlebars.register_helper(&Config::idem().view.translator_helper_name, Box::new(i18n::translate));
        handlebars.register_helper(&Config::idem().view.gravatar_helper_name, Box::new(url::gravatar));
        handlebars
    };
}

pub struct Renderer;

impl Renderer
{
    pub fn idem() -> &'static Handlebars
    {
        &HANDLEBARS
    }
}