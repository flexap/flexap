use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Debug, Deserialize, Default)]
pub struct Config
{
    pub app_name: String,
    pub base_url: String,
    pub web_path: String,
    pub db: DbConfig,
    pub view: ViewConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct DbConfig
{
    pub driver: String,
    pub host: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ViewConfig
{
    pub ext: String,
    pub templates_path: String,
    pub translator_helper_name: String,
    pub gravatar_helper_name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct SecurityConfig
{
    pub cookie_key: String,
    pub csrf_protection_key: String,
    pub csrf_token_ttl_seconds: i64,
}


lazy_static! {
    pub static ref CONFIG: Config = {
        let mut file = File::open("config.toml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let decoded: Config = toml::from_str(&contents).unwrap();
        decoded
    };
}

impl Config
{
    pub fn idem() -> &'static Config
    {
        &CONFIG
    }
}