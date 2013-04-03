extern mod std;

fn is_three(num: int) -> bool {
  return true;
}

#[test]
fn test_is_three() {
  if is_three(1) {
    fail ~"One is not three";
  }
}

fn main() {
}
