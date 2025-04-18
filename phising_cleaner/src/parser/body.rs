use regex::Regex;

pub fn extract_body(raw: &str) -> (String, Vec<String>) {
    let body_start = raw.find("\r\n\r\n").unwrap_or(0);
    let body = &raw[body_start..];

    let link_re = Regex::new(r#"https?://[^\s"'<>]+"#).unwrap();

    let mut links = Vec::new();

    for cap in link_re.captures_iter(body) {
        links.push(cap[0].to_string());
    }

    (body.trim().to_string(), links)
}
