// note - the following functions are defined in helpers.rs:
// >>> is_safe_level() 

pub fn part_two(input: &str) -> Option<u32> {
    let mut output : u32 = 0;

    for line in input.split("\n") {
        let tokens = line.split_whitespace();
        let mut nums : Vec<i32> = Vec::new();

        for token in tokens { nums.push(token.parse().expect("Invalid Token.")); }
        
        if is_safe_level(nums.clone()) { output += 1; continue; } 

        for i in 0..nums.len() {
            let mut mod_nums = nums.clone();
            mod_nums.remove(i);

            if is_safe_level(mod_nums) { output += 1; break; }
        }
    }

    Some(output)
}
