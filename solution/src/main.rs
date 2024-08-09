mod game;
mod utils;

use std::io;
use game::place_piece;

fn main() {
    // Read the player number from the game engine.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error while reading the input");
    let playerid = input.chars().nth(10).unwrap();
    let mut playersymbol = Vec::new();
    let mut ennemysymbol = Vec::new();
    let mut playercoords = Vec::new();
    let mut ennemycoords = Vec::new();
    if playerid=='1' 
    {
        playersymbol=vec!['@','a'];
        ennemysymbol=vec!['$','s']
    } else {
        playersymbol=vec!['$','s'];
        ennemysymbol=vec!['@','a']
    }

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Error while reading the input");
        let grid_details = input.split_whitespace().collect::<Vec<&str>>()[2];
        let lines = &grid_details[..grid_details.len()-1];
        let grid_lines = lines.parse::<usize>().unwrap();

        // Define the Anfield grid as a 2D vector of characters.
        let mut grid = Vec::new();

        // reading the grid
        for i in 0..grid_lines+1 {
            input.clear();
            io::stdin().read_line(&mut input).expect("Error while reading the input");
            if i<1 {
                continue;
            } else {
                let row: Vec<char> = input[4..input.len()-1].chars().collect();
                for j in 0..row.len(){
                    if playersymbol.contains(&row[j]) {
                        playercoords.push((j,i))
                    }
                    if ennemysymbol.contains(&row[j]) {
                        ennemycoords.push((j,i))
                    }
                }
                grid.push(row);
            }
        }
        
        // Defining the piece
        let mut piece = Vec::new();

        // reading the piece
        input.clear();
        io::stdin().read_line(&mut input).expect("Error while reading the input");
        let piece_details = input.split_whitespace().collect::<Vec<&str>>();
        let lines = piece_details[2];
        let piece_lines = lines[..lines.len()-1].parse::<i32>().unwrap();

        for _ in 0..piece_lines {
            input.clear();
            io::stdin().read_line(&mut input).expect("Error while reading the input");
            let row: Vec<char> = input[..input.len()-1].chars().collect();
            piece.push(row);
        }

        // finds the piece ideal position
        let (piece_x, piece_y) = place_piece(&grid, &piece, &playercoords, &ennemycoords, &playersymbol);

        // Print the coordinates of the piece placement to standard output.
        println!("{} {}", piece_x, piece_y);
   }
}