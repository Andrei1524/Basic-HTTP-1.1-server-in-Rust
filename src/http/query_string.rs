use std::{collections::HashMap, vec};

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

// here we implement a fn for query string
impl<'buf> QueryString<'buf> {
    // we pass an option because mb we dont retrieve the value
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// we use From here because this conversion cannot fail
// here we convert a str to a QueryString type
// a=1&b=2
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..]; // we need to ignore the '=' so we say +1, whic his one byte
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing { // this is a clojure
                    Value::Single(prev_val) => {
                        // here we deference, we stop getting the reference from the Single
                        *existing = Value::Multiple(vec![prev_val, val]);
                    },
                    Value::Multiple(vec) => {vec.push(val);}
                })
                .or_insert(Value::Single(val));
        }

        QueryString {data}

    }
}
