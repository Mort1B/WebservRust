use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'inBufStream> {
    data: HashMap<&'inBufStream str, Value<'inBufStream>>,
}

#[derive(Debug)]
pub enum Value<'inBufStream> {
    Single(&'inBufStream str),
    Multiple(Vec<&'inBufStream str>),
}

impl<'inBufStream> QueryString<'inBufStream> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'inBufStream> From<&'inBufStream str> for QueryString<'inBufStream> {
    fn from(s: &'inBufStream str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find("=") {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing_map| match existing_map {
                    Value::Single(old_val) => *existing_map = Value::Multiple(vec![old_val, val]),
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data: data }
    }
}
