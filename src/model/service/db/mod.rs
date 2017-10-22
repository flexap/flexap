use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::expression::sql;
use diesel::types::Text;

use model::error::Error;

use config::Config;
use model::entity::User;

pub fn connection(user: &User) -> Result<MysqlConnection, Error>
{
    let connection_url = format!("{}://{}:{}@{}/",
                                 Config::idem().db.driver,
                                 user.name,
                                 user.password,
                                 Config::idem().db.host
    );

    MysqlConnection::establish(&connection_url)
        .map_err(|err| Error::from(err))
}

pub fn list(user: &User) -> Result<Vec<String>, Error>
{
    let connection = connection(user)?;

    let query = sql::<Text>("show databases");
    let mut result = query.load::<String>(&connection)
        .map_err(|err| Error::from(err))?;

    if let Some(index) = result.iter().position(|db_name| db_name == "information_schema") {
        result.remove(index);
    }

    Ok(result)
}