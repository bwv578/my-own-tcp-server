use serde_json::Value;

pub fn decode_query(qs: &str, params:&mut Value) {
    for (k, v) in form_urlencoded::parse(qs.as_bytes()) {
        match params.as_object_mut() {
            Some(map) => {
                map.insert(k.to_string(), Value::String(v.to_string()));
            }
            _ => {}
        }
    }
}