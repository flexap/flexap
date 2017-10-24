use hyper::server::Response;
use serde_json::value::to_value as to_json_value;

use super::base;
use config::Config;
use controller::context::RequestContext;
use model::service::db;

pub fn index(request: RequestContext) -> Response
{
    let user = request.user();
    if user.is_none() {
        return base::redirect("/user/login");
    }

    let ref db_name = request.uri_path_chunks()[1];
    println!("db: {:?}", db_name);

    base::render(&request, |_request, replacements| {
        let title = format!("{} - {}", Config::idem().app_name, "DB entities");
        replacements.insert("title".to_string(), to_json_value(title)?);

        Ok("db/index".to_owned())
    })
}