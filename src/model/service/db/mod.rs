use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::expression::sql_literal::sql;
use diesel::types::Text;
use url::percent_encoding::{percent_encode, USERINFO_ENCODE_SET};

use model::error::Error;

use config::Config;
use model::entity::User;

pub struct DbService
{
    pub connection: MysqlConnection
}

impl DbService
{
    pub fn new(user: &User) -> Self
    {
        let connection = DbService::connection(user).unwrap();

        DbService {
            connection
        }
    }

    pub fn connection(user: &User) -> Result<MysqlConnection, Error>
    {
        let connection_url = format!(
            "{}://{}:{}@{}/",
            Config::idem().db.driver,
            percent_encode(user.name.as_ref(), USERINFO_ENCODE_SET),
            percent_encode(user.password.as_ref(), USERINFO_ENCODE_SET),
            Config::idem().db.host
        );

        MysqlConnection::establish(&connection_url)
            .map_err(|err| Error::from(err))
    }

    pub fn list(&self) -> Result<Vec<String>, Error>
    {
        let query = sql::<Text>("show databases");
        let mut result = query.load::<String>(&self.connection)
            .map_err(|err| Error::from(err))?;

        if let Some(index) = result.iter().position(|db_name| db_name == "information_schema") {
            result.remove(index);
        }

        Ok(result)
    }
}