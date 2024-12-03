pub fn part_two(input: &str) -> Option<u32> {
    use regex::Regex;

    let expr = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();    
    let mut do_mul : bool = true;

    let output = expr.captures_iter(input)
        .map(|caps| 
        {         
            let (op_left, op_right) = (caps.get(1), caps.get(2));

            if op_left.is_some() && op_right.is_some() && do_mul {
                let op1 = caps.get(1).unwrap().as_str();
                let op2 = caps.get(2).unwrap().as_str();

                op1.parse::<u32>().expect("Invalid Token") * op2.parse::<u32>().expect("Invalid Token")
            } else {
                let do_mul_str = caps.get(0).unwrap().as_str();

                if do_mul_str == "do()" { do_mul = true; }
                if do_mul_str == "don't()" { do_mul = false; }
                
                return 0;
            }
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
    ;

    Some(output)
}