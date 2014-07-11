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
        let your_favorite_numbers = vec!(1i, 2i, 3i);
        let my_favorite_numbers = vec!(4i, 5i, 6i);

        let our_favorite_numbers = your_favorite_numbers + my_favorite_numbers;

        println!("The third favorite number is {:d}.", *our_favorite_numbers.get(2))
    }
~~~

Seems like business as usual: `+` adds two vectors, `get()` does an
indexing operation.

Mutability inheritance
----------------------

You can mutate vectors if you make them so:

~~~ {.rust}
    fn main() {
        let mut another_vector = vec!(4i);
        another_vector.push_all([1, 2, 3]);

        println!("The second number is {:d}.", *another_vector.get(1))
    }
~~~

Of course, changing an element of a vector doesn't make sense:

~~~ {.rust}
    fn main() {
        let a_vector = vec!(1i, 2i, 3i);
        a_vector.get(0) = 5; // error: illegal left-hand side expression

        println!("The first number is {:d}.", *a_vector.get(0))
    }
~~~

But you can move it to a mutable one and then change it:

~~~ {.rust}
    fn main() {
        let a_vector = vec!(1i, 2i, 3i);
        let mut mut_vector = a_vector;
        *mut_vector.get_mut(0) = 5;

        println!("The first number is {:d}.", *mut_vector.get(0))
    }
~~~

When you make an immutable vector mutable, it's called 'thawing' the
vector, and the opposite is 'freezing' a vector.

That's it! Vectors are pretty simple.
