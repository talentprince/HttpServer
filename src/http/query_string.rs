use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        return self.get(key);
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(value: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in value.split("&") {
            let mut key = sub_str;
            let mut value = "";
            if let Some(i) = sub_str.find("=") {
                key = &sub_str[..i];
                value = &sub_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(current_value) => {
                        *existing = Value::Multiple(vec![current_value, value]);
                    }
                    Value::Multiple(vec) => vec.push(value),
                })
                .or_insert(Value::Single(value));
        }
        QueryString { data }
    }
}
