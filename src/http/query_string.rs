use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'input_buf_stream> {
    data: HashMap<&'input_buf_stream str, Value<'input_buf_stream>>,
}

#[derive(Debug)]
pub enum Value<'input_buf_stream> {
    Single(&'input_buf_stream str),
    Multiple(Vec<&'input_buf_stream str>),
}

impl<'input_buf_stream> QueryString<'input_buf_stream> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'input_buf_stream> From<&'input_buf_stream str> for QueryString<'input_buf_stream> {
    fn from(s: &'input_buf_stream str) -> Self {
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
