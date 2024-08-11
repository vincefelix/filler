mod utils;
mod game;

use game::Player;

fn main() {
    let mut player = Player::new();

    loop {
        player.initialize_identifiant();
        player.update_board();
        player.update_piece();
        player.determine_move();
    }
}
