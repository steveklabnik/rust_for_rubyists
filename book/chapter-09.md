Vectors
=======

Before getting into generic functions that could handle multiple kinds
of Monster, let's first talk about a format that you end up using them
with often: Vectors. Vectors are the 'array' in Dwemthy's Array: they're
lists of things, but unlike in Ruby, the elements must all be of the
same type. You can have any of the three kinds of pointers to vectors,
and you'll sometimes hear a borrowed pointer to a vector called a
'slice.'

Examples
--------

See if this looks familiar:

~~~ {.rust}
    fn main() {
        let your_favorite_numbers = vec![1i, 2i, 3i];
        let my_favorite_numbers = vec![4i, 5i, 6i];

        let mut our_favorite_numbers: Vec<int> = Vec::new();

        our_favorite_numbers.push_all(your_favorite_numbers.as_slice());
        our_favorite_number.push_all(my_favorite_numbers.as_slice());

        println!("The third favorite number is {}.", our_favorite_numbers[2])
    }
~~~

Seems like business as usual: `.push_all()` adds the slices, `our_favorite_numbers[2]` does an
indexing operation.

Mutability inheritance
----------------------

You can mutate vectors if you make them so:

~~~ {.rust}
    fn main() {
        let mut another_vector = vec![4i];
        another_vector.push_all([1, 2, 3]);

        println!("The second number is {}.", another_vector[1])
    }
~~~

Of course, changing an element of a vector doesn't make sense:

~~~ {.rust}
    fn main() {
        let a_vector = vec![1i, 2i, 3i];
        a_vector[0] = 5; // error: cannot borrow immutable local variable `a_vector` as mutable

        println!("The first number is {}.", a_vector[0])
    }
~~~

But you can move it to a mutable one and then change it:

~~~ {.rust}
    fn main() {
        let a_vector = vec![1i, 2i, 3i];
        let mut mut_vector = a_vector.clone();
        mut_vector[0] = 5;

        println!("The first number is {}.", mut_vector[0])
    }
~~~

When you make an immutable vector mutable, it's called 'thawing' the
vector, and the opposite is 'freezing' a vector.

That's it! Vectors are pretty simple.
