fn main() {
    let a_vector = ~[1,2,3];
    let mut mut_vector = a_vector;
    mut_vector[0] = 5;

    println(fmt!("The first number is %d.", mut_vector[0]))
}

