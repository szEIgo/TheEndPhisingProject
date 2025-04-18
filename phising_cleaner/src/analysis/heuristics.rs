use crate::parser::ParsedEmail;

pub fn score_email(email: &ParsedEmail) -> u32 {
    let mut score = 0;

    if is_suspicious_domain(&email.from) {
        score += 40;
    }

    if email.links.iter().any(|link| is_suspicious_link(link)) {
        score += 30;
    }

    if email.body_text.to_lowercase().contains("verify your account")
        || email.body_text.to_lowercase().contains("suspended")
    {
        score += 30;
    }

    score
}

fn is_suspicious_domain(from: &str) -> bool {
    !from.contains("@yourcompany.com") && !from.contains("trusted.com")
}

fn is_suspicious_link(link: &str) -> bool {
    !(link.contains("yourcompany.com") || link.contains("trusted.com"))
}
