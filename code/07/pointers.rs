use std::gc::Gc;

fn plus_one(x: &int) -> int {
    *x + 1
}

fn main() {
    let x = Gc::new(10);
    let y = ~10;

    println(plus_one(x.borrow()).to_str());
    println(plus_one(y).to_str());
}

