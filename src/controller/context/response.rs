use hyper::Response;
use hyper::header::SetCookie;

use model::entity::User;

pub struct ResponseContext
{
    pub response: Response,
    pub user: Option<User>,
    pub clean_user: bool
}

impl ResponseContext
{
    pub fn from_response(response: Response) -> Self
    {
        ResponseContext {
            response,
            user: None,
            clean_user: false
        }
    }

    pub fn set_cookie(&mut self, cookie: String)
    {
        let headers = self.response.headers_mut();

        let need_set_cookie_opt = match headers.get_mut() {
            Some(&mut SetCookie(ref mut content)) => {
                content.push(cookie);
                None
            },
            _ => Some(SetCookie(vec![cookie]))
        };
        if let Some(set_cookie) = need_set_cookie_opt {
            headers.set(set_cookie);
        }
    }
}