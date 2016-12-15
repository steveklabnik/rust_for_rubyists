fn main() {
    for num in range(1i, 101) {
        println!("{}",
            if div_by_fifteen(num) { "FizzBuzz".to_string() }
            else if div_by_three(num) { "Fizz".to_string() }
            else if div_by_five(num) { "Buzz".to_string() }
            else { num.to_string() }
        );
    }
}

fn div_by_three(num: int) -> bool {
    num % 3 == 0
}

fn div_by_five(num: int) -> bool {
    num % 5 == 0
}

fn div_by_fifteen(num: int) -> bool {
    num % 15 == 0
}


#[test]
fn test_div_by_three() {
    assert!(div_by_three(1) == false, "One is not there");
}

#[test]
fn test_div_by_three_with_three() {
    assert!(div_by_three(3) == true, "Three should be three");
}

#[test]
fn test_div_by_five() {
    assert!(div_by_five(1) == false, "One is not five");
}

#[test]
fn test_div_by_five_with_five() {
    assert!(div_by_five(5) == true, "Five should be five");
}

#[test]
fn test_div_by_fifteen() {
    assert!(div_by_fifteen(1) == false, "One is not fifteen");
}

#[test]
fn test_div_by_fifteen_with_fifteen() {
    assert!(div_by_fifteen(15) == true, "Fifteen should be fifteen");
}

