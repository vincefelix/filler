use std::io;
use std::time::Duration;
use crate::utils::{calculate_distance_between_points, is_valid_placement, get_opponent_positions};

#[derive(Debug, Clone)]
pub struct Player {
    pub identifiant: (char, char),
    pub board: Vec<Vec<char>>,
    pub piece: Vec<Vec<char>>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            identifiant: (' ', ' '),
            board: Vec::new(),
            piece: Vec::new(),
        }
    }

    pub fn initialize_identifiant(&mut self) {
        if self.identifiant.1 == ' ' {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");
            self.identifiant = if input.contains("p1") {
                ('a', '@')
            } else {
                ('s', '$')
            };
        }
    }

    pub fn update_board(&mut self) {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || input.is_empty() {
            std::thread::sleep(Duration::from_secs(10));
        }
        let row_count = input
            .replace(":", "")
            .replace("Anfield ", "")
            .split_whitespace()
            .map(|elem| elem.parse().unwrap())
            .collect::<Vec<u32>>()[1];
        let _ = io::stdin().read_line(&mut input);
        self.board = Vec::new();
        for _ in 0..row_count {
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Error reading input");
            let chars = line.split(" ").collect::<Vec<_>>()[1]
                .replace("\n", "")
                .chars()
                .collect::<Vec<char>>();
            self.board.push(chars);
        }
    }

    pub fn update_piece(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        let row_count = input
            .replace(":", "")
            .replace("Piece ", "")
            .split_whitespace()
            .map(|elem| elem.parse().unwrap())
            .collect::<Vec<u32>>()[1];
        self.piece = Vec::new();
        for _ in 0..row_count {
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Error reading input");
            let chars = line.replace("\n", "").chars().collect::<Vec<char>>();
            self.piece.push(chars);
        }
    }

    pub fn determine_move(&self) {
        let best_coordinates = calculate_best_move(self);
        println!("{} {}", best_coordinates.0, best_coordinates.1);
    }
}

fn calculate_best_move(player: &Player) -> (u32, u32) {
    let opponent_positions = get_opponent_positions(&player.board, player.identifiant);
    let mut valid_moves = Vec::new();

    for x in 0..player.board.len() {
        for y in 0..player.board[0].len() {
            if is_valid_placement(x, y, &player.board, &player.piece, player.identifiant) {
                valid_moves.push((x as i32, y as i32));
            }
        }
    }

    if !valid_moves.is_empty() && !opponent_positions.is_empty() {
        let optimal_move = select_optimal_move(valid_moves, &opponent_positions, &player.board, player.identifiant);
        return (optimal_move.1 as u32, optimal_move.0 as u32);
    }
    (0, 0)
}

fn select_optimal_move(valid_moves: Vec<(i32, i32)>, opponent_positions: &[(usize, usize)], board: &[Vec<char>], player_symbols: (char, char)) -> (i32, i32) {
    let mut best_move = valid_moves[0];

    for &move_coord in valid_moves.iter() {
        if is_preferred_move(move_coord, board, player_symbols) {
            let min_distance_to_opponent = opponent_positions.iter()
                .map(|&opp_pos| calculate_distance_between_points(move_coord, (opp_pos.0 as i32, opp_pos.1 as i32)))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            let current_min_distance = opponent_positions.iter()
                .map(|&opp_pos| calculate_distance_between_points(best_move, (opp_pos.0 as i32, opp_pos.1 as i32)))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            if min_distance_to_opponent < current_min_distance {
                best_move = move_coord;
            }
        }
    }
    best_move
}

fn is_preferred_move(coord: (i32, i32), board: &[Vec<char>], player_symbols: (char, char)) -> bool {
    let (x, y) = coord;
    if x > 0 && (board[(x - 1) as usize][y as usize] == player_symbols.0 || board[(x - 1) as usize][y as usize] == player_symbols.1) {
        return false;
    }
    if y > 0 && (board[x as usize][(y - 1) as usize] == player_symbols.0 || board[x as usize][(y - 1) as usize] == player_symbols.1) {
        return false;
    }
    if x < (board.len() as i32) - 1 && (board[(x + 1) as usize][y as usize] == player_symbols.0 || board[(x + 1) as usize][y as usize] == player_symbols.1) {
        return false;
    }
    if y < (board[0].len() as i32) - 1 && (board[x as usize][(y + 1) as usize] == player_symbols.0 || board[x as usize][(y + 1) as usize] == player_symbols.1) {
        return false;
    }
    true
}
