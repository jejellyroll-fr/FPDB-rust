use regex::Regex;

pub fn identify_site(content: &str) -> Option<String> {
    if Regex::new(r"PokerStars").unwrap().is_match(content) {
        Some("PokerStars".to_string())
    } else if Regex::new(r"Full Tilt Poker").unwrap().is_match(content) {
        Some("FullTilt".to_string())
    } else {
        None
    }
}