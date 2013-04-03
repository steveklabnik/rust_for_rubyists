fn main() {
  for [1,2,3].each |&num| {
    io::println(int::str(num))
  }
}
