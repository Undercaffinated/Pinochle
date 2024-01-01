pub struct Card {
    value: Value,
    suit: Suit,
}

enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

enum Value {
    Ace,
    Ten,
    King,
    Queen,
    Jack,
    Nine,
}

const ACE_OF_SPADES: Card = Card {
    value: Value::Ace,
    suit: Suit::Spades,
};

const TEN_OF_SPADES: Card = Card {
    value: Value::Ten,
    suit: Suit::Spades,
};

const KING_OF_SPADES: Card = Card {
    value: Value::King,
    suit: Suit::Spades,
};

const QUEEN_OF_SPADES: Card = Card {
    value: Value::Queen,
    suit: Suit::Spades,
};

const JACK_OF_SPADES: Card = Card {
    value: Value::Jack,
    suit: Suit::Spades,
};

const NINE_OF_SPADES: Card = Card {
    value: Value::Nine,
    suit: Suit::Spades,
};

const ACE_OF_DIAMONDS: Card = Card {
    value: Value::Ace,
    suit: Suit::Diamonds,
};

const TEN_OF_DIAMONDS: Card = Card {
    value: Value::Ten,
    suit: Suit::Diamonds,
};

const KING_OF_DIAMONDS: Card = Card {
    value: Value::King,
    suit: Suit::Diamonds,
};

const QUEEN_OF_DIAMONDS: Card = Card {
    value: Value::Queen,
    suit: Suit::Diamonds,
};

const JACK_OF_DIAMONDS: Card = Card {
    value: Value::Jack,
    suit: Suit::Diamonds,
};

const NINE_OF_DIAMONDS: Card = Card {
    value: Value::Nine,
    suit: Suit::Diamonds,
};

const ACE_OF_CLUBS: Card = Card {
    value: Value::Ace,
    suit: Suit::Clubs,
};

const TEN_OF_CLUBS: Card = Card {
    value: Value::Ten,
    suit: Suit::Clubs,
};

const KING_OF_CLUBS: Card = Card {
    value: Value::King,
    suit: Suit::Clubs,
};

const QUEEN_OF_CLUBS: Card = Card {
    value: Value::Queen,
    suit: Suit::Clubs,
};

const JACK_OF_CLUBS: Card = Card {
    value: Value::Jack,
    suit: Suit::Clubs,
};

const NINE_OF_CLUBS: Card = Card {
    value: Value::Nine,
    suit: Suit::Clubs,
};

const ACE_OF_HEARTS: Card = Card {
    value: Value::Ace,
    suit: Suit::Hearts,
};

const TEN_OF_HEARTS: Card = Card {
    value: Value::Ten,
    suit: Suit::Hearts,
};

const KING_OF_HEARTS: Card = Card {
    value: Value::King,
    suit: Suit::Hearts,
};

const QUEEN_OF_HEARTS: Card = Card {
    value: Value::Queen,
    suit: Suit::Hearts,
};

const JACK_OF_HEARTS: Card = Card {
    value: Value::Jack,
    suit: Suit::Hearts,
};

const NINE_OF_HEARTS: Card = Card {
    value: Value::Nine,
    suit: Suit::Hearts,
};