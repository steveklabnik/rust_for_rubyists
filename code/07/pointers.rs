fn plus_one(x: &int) -> int {
    *x + 1
}

fn main() {
    let x = @10;
    let y = ~10;

    println(plus_one(x).to_str());
    println(plus_one(y).to_str());
}

