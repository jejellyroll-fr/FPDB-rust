use super::PokerSiteParser;
use crate::hand::Hand;

pub struct PokerStarsParser;

impl PokerSiteParser for PokerStarsParser {
    fn parse(&self, _content: &str) -> Vec<Hand> {
        // Implement PokerStars specific parsing logic
        vec![]
    }
}

pub struct FullTiltParser;

impl PokerSiteParser for FullTiltParser {
    fn parse(&self, _content: &str) -> Vec<Hand> {
        // Implement Full Tilt specific parsing logic
        vec![]
    }
}