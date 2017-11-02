use super::base;
use config::Config;
use controller::context::{RequestContext, ResponseContext};
use model::service::DbService;

pub fn index(request: RequestContext) -> ResponseContext
{
    base::db_user(&request, |request, _user, db_name, db_list| {
        let db_name = db_name.unwrap();

        base::render(&request, |_request, replacements| {
            let title = format!("{} - {}", Config::idem().app_name, "DB entities");
            replacements.insert("title", title);
            replacements.insert("db_name", db_name);

            if db_list.len() > 1 {
                replacements.insert("db_list", &db_list);
            }

            Ok("db/index".to_owned())
        })
    })
}

pub fn sql(request: RequestContext) -> ResponseContext
{
    base::db_user(&request, |request, _user, db_name, db_list| {
        let db_name = db_name.unwrap();

        base::render(&request, |_request, replacements| {
            let title = format!("{} - {}", Config::idem().app_name, "SQL shell");
            replacements.insert("title", title);
            replacements.insert("db_name", db_name);

            if db_list.len() > 1 {
                replacements.insert("db_list", &db_list);
            }

            Ok("db/sql".to_owned())
        })
    })
}