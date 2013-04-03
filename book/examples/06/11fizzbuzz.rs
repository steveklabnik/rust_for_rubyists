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
