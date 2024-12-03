pub fn part_two(input: &str) -> Option<u32> {
    // get single captures for "do()" and "don't" and subcaptures for "mul(x, y)" of "x" and "y"
    let expr = regex::Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();    
    let mut do_mul : bool = true;

    let output = expr.captures_iter(input)
        .map(|caps| 
        {         
            let (op_left, op_right) = (caps.get(1), caps.get(2));
            // if both subcaptures exist it is a mul() fn, only works if do_mul is enabled 
            if op_left.is_some() && op_right.is_some() && do_mul {
                let op_left : u32 = op_left.unwrap().as_str().parse().unwrap();
                let op_right : u32 = op_right.unwrap().as_str().parse().unwrap();

                return op_left * op_right;
            } else {
                let do_mul_str = caps.get(0).unwrap().as_str();
                match do_mul_str {
                    "do()" => do_mul = true,
                    "don't()" => do_mul = false,
                    _ => ()
                };
                // any capture that isn't a mul() fn product will be returned to the map as a zero, this allows iteration to be done linearly in one pass
                return 0;
            }
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
    ;

    Some(output)
}
