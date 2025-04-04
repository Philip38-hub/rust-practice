use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Self {
        let value = rand::thread_rng().gen_range(0..=4);
        Suit::translate(value)
    }

    pub fn translate(value: u8) -> Self {
        match value {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            3 => Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }
}
impl Rank {
    pub fn random() -> Self {
        let value = rand::thread_rng().gen_range(0..=13);
        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Self {
        match value {
            0 => Rank::Ace,
            1..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value"),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}