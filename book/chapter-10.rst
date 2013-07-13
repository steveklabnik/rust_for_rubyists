{% import 'macros/ork.jinja' as ork with context %}

Vectors
=======

Before getting into generic functions that could handle multiple kinds of
Monster, let's first talk about a format that you end up using them with often:
Vectors. Vectors are the 'array' in Dwemthy's Array: they're lists of things,
but unlike in Ruby, the elements must all be of the same type. You can have any
of the three kinds of pointers to vectors, and you'll sometimes hear a borrowed
pointer to a vector called a 'slice.'

Examples
--------

See if this looks familliar::

  fn main() {
      let your_favorite_numbers = ~[1,2,3];
      let my_favorite_numbers = ~[4,5,6];

      let our_favorite_numbers = your_favorite_numbers + my_favorite_numbers;

      println(fmt!("The third favorite number is %d.", our_favorite_numbers[2]))
  }

Seems like business as usual: ``+`` adds two vectors, ``[]`` does an indexing
operation. What happens if you leave off the ``~`` s?::

  $ make
  rustc fizzbuzz.rs
  fizzbuzz.rs:5:29: 5:74 error: binary operation + cannot be applied to type `[<VI2>, .. 3]`
  fizzbuzz.rs:5   let our_favorite_numbers = your_favorite_numbers + my_favorite_numbers;
                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  error: aborting due to previous error
  make: *** [build] Error 101

Type ``[<VI2>, ..2]``? Basically, if you don't make your variable a pointer to
a vector, it's stored on the stack with a fixed size. So concatenating them
together doesn't make sense.

Mutability inheritance
----------------------

You can mutate vectors if you make them so::

  fn main() {
      let a_vector = ~[1,2,3];
      let mut another_vector = ~[];
      another_vector.push_all(a_vector);

      println(fmt!("The first number is %d.", another_vector[0]))
  }

Of course, changing an element of a vector doesn't make sense::

  fn main() {
      let a_vector = ~[1,2,3];
      a_vector[0] = 5; // fizzbuzz.rs:3:2: 3:12 error: cannot assign to immutable vec content

      println(fmt!("The first number is %d.", a_vector[0]))
  }

But you can move it to a mutable one and then change it::

  fn main() {
      let a_vector = ~[1,2,3];
      let mut mut_vector = a_vector;
      mut_vector[0] = 5;

      println(fmt!("The first number is %d.", mut_vector[0]))
  }

When you make an immutable vector mutable, it's called 'thawing' the vector,
and the opposite is 'freezing' a vector.

That's it! Vectors are pretty simple.
