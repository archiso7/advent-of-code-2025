fn problem_one(input: &str) -> i64 {
    let mut total = 0;
    let mut ranges = Vec::<i64>::new();

    for value in input.lines() {
        if value.contains("-") {
            // add from - index-1 to index+1 to ranges
        } else {
            // check if value in ranges. maybe switch ranges to a string a separate by commas,
            // adding will be slightly harder though.
        }
    }

    total
}

// fn problem_two(input: &str) -> i64 {}

fn main() {
    let input = "";
    let result = problem_one(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_part_one() {
        let result = problem_one(INPUT);
        let correct = 3;
        println!("Test result: {}", result);
        assert_eq!(result, correct);
    }
    // #[test]
    // fn test_part_two() {
    //     let result = problem_two(INPUT);
    //     let correct = 43;
    //     println!("Test result: {}", result);
    //     assert_eq!(result, correct);
    // }
}
