fn problem_one(input: &str) -> i64 {
    let ranges: Vec<&str> = input.split(",").collect();
    let mut total: i64 = 0;
    for range in ranges {
        if let Some((start, end)) = range.split_once("-") {
            if let (Ok(start_num), Ok(end_num)) = (start.parse::<i64>(), end.parse::<i64>()) {
                for i in start_num..=end_num {
                    let string_num = i.to_string();
                    let num_length_half = string_num.chars().count() / 2;
                    if string_num[..num_length_half] == string_num[num_length_half..] {
                        total += i;
                    }
                }
            }
        }
    }
    return total;
}

fn check_repeat(string: &str, sub_string: &str) -> bool {
    let string_length = string.chars().count();
    let sub_string_length = sub_string.chars().count();
    if sub_string.repeat(string_length/sub_string_length) == string {
        return true;
    }
    return false;
}

fn problem_two(input: &str) -> i64 {
    let ranges: Vec<&str> = input.split(",").collect();
    let mut total: i64 = 0;
    for range in ranges {
        if let Some((start, end)) = range.split_once("-") {
            if let (Ok(start_num), Ok(end_num)) = (start.parse::<i64>(), end.parse::<i64>()) {
                for num in start_num..=end_num {
                    let string_num = num.to_string();
                    for position in 1..string_num.chars().count() {
                        let repeatable = &string_num[..position];
                        if check_repeat(&string_num, &repeatable) {
                            total += num;
                            break;
                        }
                    }
                }
            }
        }
    }
    return total;
}

fn main() {
    let input = "8123221734-8123333968,2665-4538,189952-274622,4975-9031,24163352-24202932,1233-1772,9898889349-9899037441,2-15,2147801-2281579,296141-327417,8989846734-8989940664,31172-42921,593312-632035,862987-983007,613600462-613621897,81807088-81833878,13258610-13489867,643517-782886,986483-1022745,113493-167913,10677-16867,372-518,3489007333-3489264175,1858-2534,18547-26982,16-29,247-366,55547-103861,57-74,30-56,1670594-1765773,76-129,134085905-134182567,441436-566415,7539123416-7539252430,668-1146,581563513-581619699";

    let result = problem_two(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result = problem_one(input);
        let correct = 1227775554;
        println!("Test result: {}", result);
        assert_eq!(result, correct);
    }
    #[test]
    fn test_part_two() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result = problem_two(input);
        let correct = 4174379265;
        println!("Test result: {}", result);
        assert_eq!(result, correct);
    }

    #[test]
    fn test_check_repeat() {
        // Test repeating pattern "12"
        assert_eq!(check_repeat("1212", "12"), true);
        
        // Test repeating pattern "123"
        assert_eq!(check_repeat("123123", "123"), true);
        
        // Test single character repeated
        assert_eq!(check_repeat("1111", "1"), true);
        assert_eq!(check_repeat("aaaa", "a"), true);
        
        // Test non-repeating patterns
        assert_eq!(check_repeat("1234", "12"), false);
        assert_eq!(check_repeat("123456", "123"), false);
        
        // Test exact match (whole string)
        assert_eq!(check_repeat("12", "12"), true);
        
        // Test longer repeating pattern
        assert_eq!(check_repeat("121212", "12"), true);
        
        // Edge case: partial repeat (should be false)
        assert_eq!(check_repeat("12121", "12"), false);
    }
}
