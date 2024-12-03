pub fn part_one(input: &str) -> Option<u32> {
    use regex::Regex;

    // get subcaptures for "mul(x, y)" of "x" and "y"
    let expr = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let output = expr.captures_iter(input)
        .map(|caps| 
        { 
            // discard overall match, assign the str slice captures to variables named op1 + op2
            let (_, [op1, op2]) = caps.extract();
            // parse and return the product to the map
            op1.parse::<u32>().unwrap() * op2.parse::<u32>().unwrap
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
    ;

    Some(output)
}
