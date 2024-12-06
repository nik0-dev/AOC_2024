fn get_char(grid: &Vec<Vec<char>>, pos: (isize, isize), dir: (isize, isize)) -> Option<&char> {
    let (pos_x, pos_y) = (pos.0 + dir.0, pos.1 + dir.1); 

    if pos_x >= 0 && pos_y >= 0 {
        grid.get(pos_y as usize)?.get(pos_x as usize)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut out : u32 = 0;

    let (kw, rkw) = ("MAS", "SAM");
    let grid : Vec<Vec<char>> = input.split_whitespace().map(|line| line.chars().collect()).collect();
    
    let (down_left, up_right) = ((1,-1), (-1,1));
    let (up_left, down_right) = ((-1,-1), (1,1));

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = (x as isize, y as isize);

            if ch == 'A' {
                let mut diagonal = String::new();
                let mut counter_diagonal = String::new();

                diagonal.push(*get_char(&grid, pos, down_left).unwrap_or(&' '));
                diagonal.push('A');
                diagonal.push(*get_char(&grid, pos, up_right).unwrap_or(&' '));

                counter_diagonal.push(*get_char(&grid, pos, up_left).unwrap_or(&' '));
                counter_diagonal.push('A');
                counter_diagonal.push(*get_char(&grid, pos, down_right).unwrap_or(&' '));

                let is_valid = | w: String | -> bool { w == kw || w == rkw }; 
                
                if is_valid(diagonal) && is_valid(counter_diagonal) { out += 1; } 
            }
        } 
    }

    Some(out)
}