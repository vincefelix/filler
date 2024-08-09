pub fn calcul_min_distance(piece: &Vec<Vec<char>>, ennemycoords: &Vec<(usize,usize)>,(xg,yg):(usize,usize), distance:f32) -> f32 {
    let playerows = piece[0].len(); 
    let mut min_dist=distance;

    for yp in 0..piece.len(){
        for xp in 0..playerows {
            if piece[yp][xp] != '.' {
                for (xe,ye) in ennemycoords {
                    let  dist=(((*ye as f32)-((yp + yg) as f32) ).powf(2.) + ((*xe as f32)-((xp + xg) as f32)).powf(2.)).sqrt();
                    if dist < min_dist{
                        min_dist = dist;
                    }
                }
            }
        }
    }
     
    return min_dist
}

pub fn placable_piece(grid: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, pchars: &Vec<char>, xg: usize, yg: usize) -> bool {
    let mut cross = 0; 
    let mut stop = false;
    let playerows = piece[0].len(); 

    for yp in 0..piece.len(){
        for xp in 0..playerows {
            if piece[yp][xp] != '.' {
                if pchars.contains(&grid[yg+yp][xg+xp]) {
                    cross +=1;
                    if cross>1 {
                        stop = true;
                        break;
                    }
                } else if grid[yg+yp][xg+xp] != '.' {
                    stop = true;
                    break;
                }
            }
        }
        if stop {
            break
        }
    }
    if cross==1 && !stop {
        return  true;
    }
    false
}