use stdio::io;

struct Player {
    // Human or Computer Player
    player_type: Player_Type,

    // Player Name
    name: String,

    // The player's hand
    hand: Hand,
    
    // The player's bid
    bid: Bid,

    // The player's meld
    meld: u8,
}

impl Player {
    fn new(players_made: u8) -> Player {
        Player {
            player_type = Player::get_player_type(players_made),
            name = get_player_name(),
            hand = make_empty_hand(),
            bid = Undeclared,
            meld = 0,
        }
    }

    fn get_player_type(players_made: u8) {
        let mut pt = player_type::Undeclared;
        let mut input = String::new();
        while pt == player_type::Undeclared {
            println!("Is Player {} a Human (H) or Computer? (C)", players_made + 1);
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");

            pt = match input.trim() {
                'H' => Human,
                'C' => Computer,
                _ => Undeclared,
            }
        }
        pt
    }
}

enum Player_Type {
    Human,
    Computer,
    Undeclared,
}