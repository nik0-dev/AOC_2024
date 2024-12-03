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
