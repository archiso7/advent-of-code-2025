fn problem_one(input: &str) -> i64 {
    let mut total = 0;
    for (l, line) in input.lines().iter().enumerate() {
        for (c, char) in line.chars().collect().iter().enumerate() {
            if char == "@" {
                let mut surrounding = 0;
                for i in 0..3 {
                    for j in 0..3 {
                        if input.as_bytes()[]
                    }
                }
            }
        }
    }
}

// fn problem_two(input: &str) -> i64 {
// }

fn main() {
    let input = "";
    let result = problem_two(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_one() {
        let result = problem_one(input);
        let correct = 13;
        println!("Test result: {}", result);
        assert_eq!(result, correct);
    }
    // #[test]
    // fn test_part_two() {
    //     let result = problem_two(input);
    //     let correct = ;
    //     println!("Test result: {}", result);
    //     assert_eq!(result, correct);
    // }
}
