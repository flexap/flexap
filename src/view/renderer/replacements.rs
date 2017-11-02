use serde::{Serialize, Serializer};
use serde_json::value::{Map, Value};
use handlebars::to_json;

type Inner = Map<String, Value>;

pub struct TemplateReplacements(Inner);

impl TemplateReplacements
{
    pub fn new() -> Self
    {
        TemplateReplacements(Map::new())
    }

    #[inline]
    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<Value>
        where K: Into<String>, V: Serialize
    {
        self.0.insert(key.into(), to_json(&value))
    }
}

impl Serialize for TemplateReplacements
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.0.serialize(serializer)
    }
}
