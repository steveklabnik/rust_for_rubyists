extern mod std;

fn is_three(num: int) -> bool {
  num % 3 == 0
}

#[test]
fn test_is_three_with_not_three() {
  assert !is_three(1)
}

#[test]
fn test_is_three_with_three() {
  assert is_three(3)
}

fn is_five(num: int) -> bool {
  num % 5 == 0
}

#[test]
fn test_is_five_with_not_five() {
  assert !is_five(1)
}

#[test]
fn test_is_five_with_five() {
  assert is_five(5)
}

fn is_fifteen(num: int) -> bool {
  num % 15 == 0
}

#[test]
fn test_is_fifteen_with_not_fifteen() {
  assert !is_fifteen(1)
}

#[test]
fn test_is_fifteen_with_fifteen() {
  assert is_fifteen(15)
}


fn main() {
  for int::range(1, 101) |num| {
    io::println(
      if is_fifteen(num) { ~"FizzBuzz" }
      else if is_three(num) { ~"Fizz" }
      else if is_five(num) { ~"Buzz" }
      else { int::str(num) }
    );
  }
}
