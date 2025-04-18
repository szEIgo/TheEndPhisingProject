

use analysis::heuristics::score_email;
use parser::parse_email;

mod analysis;
mod parser;

fn main() {
    let email_raw = include_str!("../samples/sample_email.eml");
    let email = parse_email(email_raw);
    let score = score_email(&email);

    println!("Email from: {}", email.from);
    println!("Subject: {}", email.subject);
    println!("Links: {:?}", email.links);
    println!("--- Score: {} ---", score);

    if score > 50 {
        println!("⚠️ Suspicious email detected!");
    } else {
        println!("✅ Looks legit.");
    }
}
