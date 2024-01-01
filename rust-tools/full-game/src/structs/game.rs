use crate::structs::player::Player;

pub struct Game {
    // The four players
    north: Player,
    south: Player,
    east: Player,
    west: Player,

    // Number of human and CPU players
    // num_humans: PlayerCount,
    // num_computers: PlayerCount,

    // The current score
    north_south_score: i32,
    east_west_score: i32,

    // Who dealt last
    who_dealt_last: Player,

    // Tracks if the game has been won
    playing: bool,
}

impl Game {
    fn new() -> Game {
        // players_made helps govern the behavior
        // of some functions.
        let mut players_made: u8 = 0;

        Game {
            north: Player::new(),
            south: Player::new(),
            east: Player::new(),
            west: Player::new(),
            // num_humans: PlayerCount::Zero,
            // num_computers: PlayerCount::Zero,
            north_south_score: 0,
            east_west_score: 0,
            playing: true,
            who_dealt_last: west.name,
        }
    }

    fn print_game_state(&self) {
        println!("Team 1");
        println!("{} and {}", self.north.name, self.south.name);
        println!("Score: {}", self.north_south_score);
        println!("");
        println!("Team 2");
        println!("{} and {}", self.east.name, self.west.name);
        println!("Score: {}", self.east_west_score);
        println!("");
        println!("{}, dealt last", self.who_dealt_last.name);
    }
}