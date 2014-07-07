fn main() {
    for num in range(1u, 101) { 
        println( 
            if is_fifteen(num) { ~"FizzBuzz" } 
            else if is_three(num) { ~"Fizz" } 
            else if is_five(num) { ~"Buzz" } 
            else { num.to_str() } 
            ); 
    } 
}

fn is_three(num: int) -> bool { 
    num % 3 == 0
} 

fn is_five(num: int) -> bool { 
    num % 5 == 0
} 

fn is_fifteen(num: int) -> bool { 
    num % 15 == 0
} 


#[test]
fn test_is_three() {
    if is_three(1) {
        fail!("One is not three");
    }
}

#[test]
fn test_is_three_with_three() {
    if !is_three(3) {
        fail!("Three should be three");
    }
}

#[test]
fn test_is_five() {
    if is_five(1) {
        fail!("One is not five");
    }
}

#[test]
fn test_is_five_with_five() {
    if !is_five(5) {
        fail!("Five should be five");
    }
}

#[test]
fn test_is_fifteen() {
    if is_fifteen(1) {
        fail!("One is not fifteen");
    }
}

#[test]
fn test_is_fifteen_with_fifteen() {
    if !is_fifteen(15) {
        fail!("Fifteen should be fifteen");
    }
}

