ub fn part_two(input: &str) -> Option<u32> {
    use regex::Regex;

    let expr = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();    
    let mut do_mul : bool = true;

    let output = expr.captures_iter(input)
        .map(|caps| 
        {         
            let (op_left, op_right) = (caps.get(1), caps.get(2));

            if op_left.is_some() && op_right.is_some() && do_mul {
                let op_left : u32 = op_left.unwrap().as_str().parse::<u32>().expect("Invalid Token");
                let op_right : u32 = op_right.unwrap().as_str().parse::<u32>().expect("Invalid Token");

                return op_left * op_right;
            } else {
                let do_mul_str = caps.get(0).unwrap().as_str();

                match do_mul_str {
                    "do()" => do_mul = true,
                    "don't()" => do_mul = false,
                    _ => ()
                };

                return 0;
            }
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
    ;

    Some(output)
}
