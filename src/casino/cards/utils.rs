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

    pub fn from_short(short: String) -> Result<Self, InvalidArgument> {
        if short.len() != 2 {
            return Err(InvalidArgument::InvalidArgument);
        }

        let mut split_short = short.as_str().chars();

        let short_rank = split_short.next().unwrap().to_string();
        let short_suite  = split_short.next().unwrap().to_string();

        let suite = match short_suite.to_lowercase().as_str() {
            "c" => Suite::CLUBS,
            "d" => Suite::DIAMONDS,
            "h" => Suite::HEARTS,
            "s" => Suite::SPADES,
            _ => {return Err(InvalidArgument::InvalidArgument);}
        };

        let rank = match short_rank.to_uppercase().as_str() {
            "A" => Rank::ACE,
            "K" => Rank::KING,
            "Q" => Rank::QUEEN,
            "J" => Rank::JACK,
            "T" => Rank::TEN,
            _ => Rank::NUMERAL(short_rank.parse::<u8>().unwrap())
        };

        Self::new(suite, rank)
    }
}
