struct Trick {
    // The cards played in this trick
    cards: [Card; 4],

    // The player who won this trick
    winner: Player::name,

    // Who played first
    first_to_play: Player::name,

    // The trump suit
    trump: Suit,

    // The suit that was led
    suit_led: Suit,

    // Counters in the trick
    counters_in_trick: Zero_To_Four_Inclusive,

}

impl Trick {

}