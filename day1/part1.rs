pub fn part_one(input: &str) -> Option<u32> {    
    let mut left : Vec<u32> = Vec::new();
    let mut right : Vec<u32> = Vec::new();
    let mut res : u32 = 0;

    for (i, token) in input.split_whitespace().into_iter().enumerate() {
        let num : u32 = token.parse().expect("Invalid Token.");
        if i % 2 == 0 { left.push(num) } else { right.push(num); }
    }
    
    left.sort();
    right.sort();

    for i in 0..left.len() { res += left[i].abs_diff(right[i]); }

    Some(res)
}