pub fn part_two(input: &str) -> Option<u32> {
    use std::collections::HashMap;

    let mut map : HashMap<u32, u32> = HashMap::new();
    let mut vec : Vec<u32> = Vec::new();

    for (i, token) in input.split_whitespace().into_iter().enumerate() {
        let num : u32 = token.parse().expect("Invalid Token.");
        
        let is_left : bool = i % 2 == 0;

        if is_left { 
            vec.push(num); 
            if map.get(&num).is_none() { map.insert(num, 0); }
        } else {
            match map.get(&num) {
                Some(count) => { map.insert(num, count + 1); },
                None => { map.insert(num, 1); }
            }
        }
    };    

    Some(vec.iter().map(|n| n * map[n]).sum())
}