use controller::context::BodyContent;
use model::entity::User;

#[derive(Default, Debug)]
pub struct LoginFormDto
{
    pub login: String,
    pub password: String,
    pub rememberme: bool
}

impl LoginFormDto
{
    pub fn form_name() -> &'static str
    {
        "login-form"
    }

    pub fn is_none(&self) -> bool
    {
        self.login.is_empty() || self.password.is_empty()
    }

    pub fn user(self) -> User
    {
        User {
            name: self.login,
            password: self.password
        }
    }
}

impl<'a> From<&'a BodyContent> for LoginFormDto
{
    fn from(body: &'a BodyContent) -> Self
    {
        let mut dto = LoginFormDto::default();
        if let Some(login) = body.get(&format!("{}[login]", LoginFormDto::form_name())) {
            dto.login = login.to_string();
        }
        if let Some(password) = body.get(&format!("{}[password]", LoginFormDto::form_name())) {
            dto.password = password.to_string();
        }
        if let Some(_) = body.get(&format!("{}[rememberme]", LoginFormDto::form_name())) {
            dto.rememberme = true;
        }
        dto
    }
}