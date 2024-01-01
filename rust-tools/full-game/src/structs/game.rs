struct Game {
    // The four players
    north: Player,
    south: Player,
    east: Player,
    west: Player,

    // Number of human and CPU players
    num_humans: PlayerCount,
    num_computers: PlayerCount,

    // The current score
    north_south_score: i32,
    east_west_score: i32,

    // Who dealt last
    dealer: Player,

    // Tracks if the game has been won
    playing: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            north: Player::new(),
            south: Player::new(),
            east: Player::new(),
            west: Player::new(),
            num_humans: PlayerCount::Zero,
            num_computers: PlayerCount::Zero,
            north_south_score: 0,
            east_west_score: 0,
            dealer: Player::new(),
            playing: true,
        }
    }
}