pub fn part_one(input: &str) -> Option<u32> {
    use regex::Regex;

    let expr = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let output = expr.captures_iter(input)
        .map(|caps| 
        { 
            let (_, [op1, op2]) = caps.extract();
            op1.parse::<u32>().expect("Invalid Token") * op2.parse::<u32>().expect("Invalid Token")
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
    ;

    Some(output)
}