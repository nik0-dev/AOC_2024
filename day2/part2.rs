pub fn is_safe_level(vec: Vec<i32>) -> bool {
    let mut iter = vec.iter();
    let mut sign : i32 = 0;

    let mut last : i32 = match iter.next() {
        Some(val) => *val,
        None => return false
    };

    for num in iter {            
        let updated_sign : i32 = if last <= *num { 1 } else { -1 };
        let diff : u32 = num.abs_diff(last);
        
        let invalid_level_adjustment : bool = sign != 0 && sign != updated_sign || diff < 1 || diff > 3;
        if invalid_level_adjustment { return false; } 

        sign = updated_sign;
        last = *num;
    }
    
    return true;
}

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