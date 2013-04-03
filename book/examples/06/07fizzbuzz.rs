fn main() {
  for int::range(1, 101) |num| {
    let mut answer;
    if is_fifteen(num){
      answer = "FizzBuzz";
    }
    else if is_three(num) {
      answer = "Fizz";
    }
    else if is_five(num) {
      answer = "Buzz";
    }
    else {
      answer = "";
    };
    io::println(answer)
  }
}
