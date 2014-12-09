fn main() {
    for num in range(1i, 101) {
        println!("{:s}",
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
    if div_by_three(1) {
        panic!("One is not three");
    }
}

#[test]
fn test_div_by_three_with_three() {
    if !div_by_three(3) {
        panic!("Three should be three");
    }
}

#[test]
fn test_div_by_five() {
    if div_by_five(1) {
        panic!("One is not five");
    }
}

#[test]
fn test_div_by_five_with_five() {
    if !div_by_five(5) {
        panic!("Five should be five");
    }
}

#[test]
fn test_div_by_fifteen() {
    if div_by_fifteen(1) {
        panic!("One is not fifteen");
    }
}

#[test]
fn test_div_by_fifteen_with_fifteen() {
    if !div_by_fifteen(15) {
        panic!("Fifteen should be fifteen");
    }
}

