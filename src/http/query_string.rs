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
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(value: &'buf str) -> Self {
        let mut data = HashMap::new();

        // query string을 &로 split하여 각각의 sub_str에 대해 반복
        for sub_str in value.split('&') {
            let mut key = sub_str;
            let mut val = "";

            // sub_str에서 = 문자의 인덱스를 찾아 key와 val로 분리
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                // HashMap에서 key에 해당하는 값이 있을 경우
                .and_modify(|existing: &mut Value| match existing {
                    // Value::Single(&str)인 경우 Value::Multiple(Vec)로 변경하고
                    // Vec에 기존 값과 새로운 값 추가
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    // Value::Multiple인 경우 Vec에 새로운 값 추가
                    Value::Multiple(vec) => vec.push(val),
                })
                // HashMap에서 key에 해당하는 값이 없을 경우
                // Value::Single(&str)로 HashMap에 새로운 값 추가
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
