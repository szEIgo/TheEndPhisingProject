use regex::Regex;

pub fn parse_headers(raw: &str) -> (String, String) {
    let from_re = Regex::new(r"(?i)^From:\s*(.+)$").unwrap();
    let subject_re = Regex::new(r"(?i)^Subject:\s*(.+)$").unwrap();

    let mut from = String::new();
    let mut subject = String::new();

    for line in raw.lines() {
        if let Some(caps) = from_re.captures(line) {
            from = caps.get(1).map_or("", |m| m.as_str()).to_string();
        }
        if let Some(caps) = subject_re.captures(line) {
            subject = caps.get(1).map_or("", |m| m.as_str()).to_string();
        }

        if !from.is_empty() && !subject.is_empty() {
            break;
        }
    }

    (from, subject)
}
