pub fn get_all_lines(grid: &Vec<Vec<char>>) -> Vec<String> {
    let (horizontal, vertical, diagonal, cross_diagonal) = ((1,0),(0,1),(1,-1),(1,1));
    let (grid_size_x, grid_size_y) = (grid[0].len(), grid.len());
    let mut lines : Vec<String> = Vec::new();
    
    // gets horizontal lines (LTR)
    for y in 0..grid_size_y { lines.push(get_line(&grid, (0, y), horizontal)); }
    // gets vertical lines (LTR)
    for x in 0..grid_size_x { lines.push(get_line(&grid, (x, 0), vertical)); }
    // gets the top + bottom half diagonals (LTR)
    for y in 0..grid_size_y { lines.push(get_line(&grid, (0, y), diagonal)); }
    for x in 1..grid_size_x { lines.push(get_line(&grid, (x, grid_size_y - 1), diagonal)); }
    // gets the top + bottom half counter-diagonals (RTL)
    for x in 0..grid_size_x { lines.push(get_line(&grid, (grid_size_x - 1 - x, 0), cross_diagonal)); }
    for y in 1..grid_size_y { lines.push(get_line(&grid, (0, y), cross_diagonal)); }
    
    return lines;
}

// iterates over the grid in a direction pushing each character to a string until it can't, and returns the final string
pub fn get_line(grid: &Vec<Vec<char>>, start: (usize, usize), dir: (i32, i32)) -> String {
    let mut x : usize = start.0; 
    let mut y : usize = start.1;

    let mut out : String = String::new();

    while y < grid.len() && x < grid[y].len() { 
        out.push(grid[y][x]);
        x = (x as i32 + dir.0) as usize; 
        y = (y as i32 + dir.1) as usize; 
    }

    return out;
}