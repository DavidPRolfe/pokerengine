/// Module of cards, hands, and how to evaluate them against each other

use std::cmp::Ordering;

pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl PartialEq for Card {
    /// Eq is being defined off of suit not mattering.
    ///
    /// This is not true for all card games but is for Texas Hold'em
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Card {}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    /// cmp is being defined off of suit not mattering.
    ///
    /// This is not true for all card games but is for Texas Hold'em
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}

#[derive(Eq, Ord, PartialOrd, PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[cfg(test)]
mod test {
    use super::*;

    /// This test exists because I made a mistake with ordering.
    #[test]
    fn rank_ord_test(){
        let bigger = Rank::Queen;
        let smaller = Rank::Seven;

        assert!(bigger > smaller);
    }

    #[test]
    fn rank_eq_test(){
        assert!(Rank::Ace == Rank::Ace);
        assert!(Rank::Jack != Rank::Five);
    }

    #[test]
    fn card_ord_test(){
        let bigger = Card { suit: Suit::Club, rank: Rank::King };
        let smaller = Card { suit: Suit::Spade, rank: Rank::Three};

        assert!(bigger > smaller);
    }
}