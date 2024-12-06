pub fn part_one(input: &str) -> Option<u32> {   
    let (kw, rkw) = ("XMAS", "SAMX"); 
    let grid : Vec<Vec<char>> = input.split_whitespace().map(|line| line.chars().collect()).collect();

    // produces lines for all directions (horizontal, vertical, diagonal, and counter-diagonal) 
    let lines : Vec<String> = get_all_lines(&grid);
    let matches = |s : &String, w: &str| -> u32 { s.matches(w).count() as u32 };

    Some(lines.iter().map(|s| matches(s, kw) + matches(s, rkw)).sum())
}