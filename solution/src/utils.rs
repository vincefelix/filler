pub fn calculate_distance_between_points(point1: (i32, i32), point2: (i32, i32)) -> f32 {
    let dx = (point2.0 - point1.0) as f32;
    let dy = (point2.1 - point1.1) as f32;
    (dx * dx + dy * dy).sqrt()
}

pub fn is_valid_placement(
    x: usize,
    y: usize,
    board: &[Vec<char>],
    piece: &[Vec<char>],
    player_symbols: (char, char),
) -> bool {
    let mut overlap_count = 0;
    let piece_height = piece.len();
    let piece_width = piece[0].len();
    let board_height = board.len();
    let board_width = board[0].len();

    for i in 0..piece_height {
        for j in 0..piece_width {
            let board_x = x + i;
            let board_y = y + j;
            if board_x >= board_height || board_y >= board_width {
                return false;
            }
            let board_cell = board[board_x][board_y];
            if piece[i][j] != '.' {
                if board_cell == player_symbols.0 || board_cell == player_symbols.1 {
                    overlap_count += 1;
                } else if board_cell != '.' {
                    return false;
                }
            }
        }
    }
    overlap_count == 1
}

pub fn get_opponent_positions(board: &[Vec<char>], player_symbols: (char, char)) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            let cell = board[x][y];
            if cell != '.' && cell != player_symbols.0 && cell != player_symbols.1 {
                positions.push((x, y));
            }
        }
    }
    positions
}
