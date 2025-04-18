pub mod headers;
pub mod body;

use headers::parse_headers;
use body::extract_body;

#[derive(Debug)]
pub struct ParsedEmail {
    pub from: String,
    pub subject: String,
    pub body_text: String,
    pub links: Vec<String>,
}

pub fn parse_email(raw: &str) -> ParsedEmail {
    let (from, subject) = parse_headers(raw);
    let (body_text, links) = extract_body(raw);

    ParsedEmail {
        from,
        subject,
        body_text,
        links,
    }
}
