extern mod std;

#[test]
fn test_is_three() {
  if is_three(1) {
    fail ~"One is not three";
  }
}

fn main() {
}
