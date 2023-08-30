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
        Ok(_) => panic!(),
        Err(e) => {assert_eq!(e, InvalidArgument::InvalidArgument)}
    };

    let my_card = Card::new(Suite::CLUBS, Rank::NUMERAL(10));

    match my_card {
        Ok(_) => panic!(),
        Err(e) => {assert_eq!(e, InvalidArgument::InvalidArgument)}
    };
}

#[test]
fn test_from_short() {
    let my_card = Card::from_short(String::from("Kh")).unwrap();

    assert_eq!(my_card, Card::new(Suite::HEARTS, Rank::KING).unwrap());


    let my_card = Card::from_short(String::from("7s")).unwrap();

    assert_eq!(my_card, Card::new(Suite::SPADES, Rank::NUMERAL(7)).unwrap());

}

#[test]
fn test_from_short_error() {
    let my_card = Card::from_short(String::from(""));

    match my_card {
        Ok(_) => panic!(),
        Err(e) => assert_eq!(e, InvalidArgument::InvalidArgument)
    };
}

#[test]
fn test_deck() {
    let my_deck = Deck::new(1);

    assert_eq!(my_deck.cards.len(), 52);
    assert_eq!(my_deck.dealt.len(), 0);


    let my_deck = Deck::new(2);

    assert_eq!(my_deck.cards.len(), 104);
    assert_eq!(my_deck.dealt.len(), 0);
}

#[test]
fn test_deck_deal() {
    let mut my_deck = Deck::new(1);

    let my_card = my_deck.deal().unwrap();

    assert_eq!(my_deck.cards.len(), 51);
    assert_eq!(my_deck.dealt.len(), 1);

    assert!(!my_deck.cards.contains(&my_card));
    assert!(my_deck.dealt.contains(&my_card));
}

#[test]
fn test_deck_shuffle() {
    let mut my_deck = Deck::new(1);
    let copy = my_deck.clone();

    assert_eq!(my_deck, copy);

    my_deck.shuffle();

    assert_ne!(my_deck, copy);

    // Test that when shuffling, all cards from 'dealt' are put back into 'cards'
    my_deck.deal();
    my_deck.shuffle();

    assert_eq!(my_deck.cards.len(), 52);
    assert_eq!(my_deck.dealt.len(), 0);
}
