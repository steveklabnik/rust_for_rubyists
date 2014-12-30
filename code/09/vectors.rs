fn main() {
    let a_vector = vec![1i, 2i, 3i];
    let mut mut_vector = a_vector.clone();
    mut_vector[0] = 5;

    println!("The first number is {}.", mut_vector[0])
}
