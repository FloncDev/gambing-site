use crate::casino::cards::utils::*;


#[test]
fn test_card_traits() {
    let mut my_card: Card = Card::new(Suite::DIAMONDS, Rank::ACE).unwrap();

    assert_eq!(my_card.suite, Suite::DIAMONDS);
    assert_eq!(my_card.rank, Rank::ACE);
    assert_eq!(my_card.short, String::from("Ad"));

    my_card = Card::new(Suite::CLUBS, Rank::NUMERAL(3)).unwrap();

    assert_eq!(my_card.rank, Rank::NUMERAL(3));
    assert_eq!(my_card.short, String::from("3c"));

    my_card = Card::new(Suite::HEARTS, Rank::TEN).unwrap();

    assert_eq!(my_card.short, String::from("Th"));
}

#[test]
fn test_card_error() {
    let my_card = Card::new(Suite::CLUBS, Rank::NUMERAL(1));

    match my_card {
        Ok(_) => {panic!()},
        Err(e) => {assert_eq!(e, InvalidArgument::InvalidArgument)}
    };

    let my_card = Card::new(Suite::CLUBS, Rank::NUMERAL(10));

    match my_card {
        Ok(_) => {panic!()},
        Err(e) => {assert_eq!(e, InvalidArgument::InvalidArgument)}
    };
}