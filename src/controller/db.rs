use serde_json::value::to_value as to_json_value;

use super::base;
use config::Config;
use controller::context::{RequestContext, ResponseContext};
use model::service::DbService;

pub fn index(request: RequestContext) -> ResponseContext
{
    base::db_user(&request, |request, _user, db_list| {
        let ref db_name = request.uri_path_chunks[1];

        if !db_list.contains(db_name) {
            println!("ERROR db::index - db with name '{}' is not allowed", db_name);
            return base::redirect("/error");
        }

        base::render(&request, |_request, replacements| {
            let title = format!("{} - {}", Config::idem().app_name, "DB entities");
            replacements.insert("title".to_string(), to_json_value(title)?);
            replacements.insert("db_name".to_string(), to_json_value(db_name)?);

            if db_list.len() > 1 {
                replacements.insert("db_list".to_string(), to_json_value(&db_list)?);
            }

            Ok("db/index".to_owned())
        })
    })
}