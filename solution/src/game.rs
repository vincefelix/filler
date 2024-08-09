use crate::utils::{placable_piece, calcul_min_distance};

pub fn place_piece(grid: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, playercoords: &Vec<(usize,usize)>, ennemycoords: &Vec<(usize,usize)>, playerchars: &Vec<char>) -> (usize, usize) {
    let gridrows = grid[0].len(); 
    let piecerows = piece[0].len();
    let  mut distance = ((gridrows as f32).powf(2.) + (grid.len() as f32).powf(2.)).sqrt();

    let mut sol = (0,0);
    let (mut xmin,mut xmax,mut ymin,mut ymax) = (grid.len(),0,gridrows,0);
    for (xg,yg) in playercoords {
        if xg < &xmin {xmin=*xg}
        if xg > &xmax {xmax=*xg}
        if yg < &ymin {ymin=*yg}
        if yg > &ymax {ymax=*yg}
    }

    let (mut xi,mut xf,mut yi,mut yf) = (0,gridrows-piecerows+1,0,grid.len()-piece.len()+1);
    let temp = xmin as i32 - piecerows as i32- 1;
    if (temp) > 0 {xi = xmin - piecerows + 1}
    if (xmax + piecerows - 1) < gridrows {xf = xmax + 1}
    let temp = ymin as i32 - piece.len() as i32 + 1;
    if (temp) > 0 {yi = ymin - piece.len() }
    if (ymax + piece.len() - 1) < grid.len() {yf = ymax }

    // l'idee est d'essayer de placer la piece partout dans la grille où se trouve le joueur
    // pour trouver toutes les solutions possibles
    for yg in yi..yf {
        for xg in xi..xf {
            if placable_piece(grid, piece, playerchars, xg, yg) {
                let min_dist = calcul_min_distance(piece, ennemycoords, (xg,yg), distance);
                if min_dist < distance {
                    distance = min_dist;
                    sol = (xg,yg);
                }
            }
        }
    }

    return sol
}