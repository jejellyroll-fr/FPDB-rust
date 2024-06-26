use crate::hand::Hand;

pub trait PokerSiteParser {
    fn parse(&self, content: &str) -> Vec<Hand>;
}

mod poker_sites;