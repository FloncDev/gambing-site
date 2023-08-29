use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Suite {
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES
}

impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    ACE,
    KING,
    QUEEN,
    JACK,
    TEN,
    NUMERAL(u8)
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suite: Suite,
    pub rank: Rank,
    pub short: String
}

#[derive(Debug, PartialEq)]
pub enum InvalidArgument { InvalidArgument }

impl Card {
    pub fn new(suite: Suite, rank: Rank) -> Result<Self, InvalidArgument> {
        let rank_short = match rank {
            Rank::NUMERAL(num) => {
                if num <= 1 || num >= 10 {
                    return Err(InvalidArgument::InvalidArgument);
                }

                num.to_string()
            },
            _ => {
                rank.to_string().as_str().chars().next().unwrap().to_string()
            }
        };

        let suite_short = suite.to_string().as_str().chars().next().unwrap().to_lowercase().to_string();

        let short = format!("{}{}", rank_short, suite_short);

        Ok(Card{
            suite,
            rank,
            short
        })
    }
}