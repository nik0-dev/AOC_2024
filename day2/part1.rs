pub fn part_one(input: &str) -> Option<u32> {
    let mut output : u32 = 0;

    for line in input.split("\n") {
        let mut tokens = line.split_whitespace();

        let mut sign : i32 = 0;
        let mut valid_level : bool = true;

        let mut last : i32 = match tokens.next() {
            Some(token) => token.parse::<i32>().expect("Invalid Token."), 
            None => break
        };

        for token in tokens {
            let num : i32 = token.parse().expect("Invalid Token.");
            
            let updated_sign : i32 = if last <= num { 1 } else { -1 };
            let diff : u32 = num.abs_diff(last);

            if sign != 0 && sign != updated_sign || diff < 1 || diff > 3 { valid_level = false; break; }

            sign = updated_sign;
            last = num;
        }

        if valid_level { output += 1; }
    }

    Some(output)
}
