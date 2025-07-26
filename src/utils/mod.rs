pub mod inbound_generator;

pub fn url_decode(value: Option<String>) -> Option<String> {
    return value.and_then(|s| {
        urlencoding::decode(&s)
            .ok()
            .map(|decoded| decoded.into_owned())
    });
}

pub fn parse_raw_json(input: &str) -> Option<serde_json::Value> {
    serde_json::from_str::<serde_json::Value>(input)
        .ok()
        .and_then(|v| match v {
            serde_json::Value::Object(_) => Some(v),
            _ => None,
        })
}

pub fn get_parameter_value(query: &Vec<(&str, &str)>, param: &str) -> Option<String> {
    return query
        .iter()
        .find(|q| String::from(q.0) == String::from(param))
        .map(|q| q.1.to_string());
}
