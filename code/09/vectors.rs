fn main() {
    let a_vector = vec!(1i, 2i, 3i);
    let mut mut_vector = a_vector;
    *mut_vector.get_mut(0) = 5;

    println!("The first number is {:d}.", *mut_vector.get(0))
}
