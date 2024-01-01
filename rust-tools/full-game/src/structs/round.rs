struct Round {
    // The current trump suit
    trump: Suit,

    // The current bid
    bid: Bid,

    // The player who won the bid
    bidder: Player,

    // The current player
    current_player: Player,

    // Scoring
    team_1_meld: u8,
    team_2_meld: u8,

    team_1_points_from_play: u8,
    team_2_points_from_play: u8,

    team_1_earned_points: u8,
    team_2_earned_points: u8,
}

impl Round {
    
}