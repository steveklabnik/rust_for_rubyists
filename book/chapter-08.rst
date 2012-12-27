{% import 'macros/ork.jinja' as ork with context %}

Boxes, Pointers, and ownership, oh my!
======================================

Since you program in Ruby, you probably don't know about pointers, nor care.
If you're going to work in a language like Rust, though, you gotta know about
them. So let's talk about the concept real quick, then dicuss how Rust handles
pointers.

Pointer recap
-------------

When you create a variable, you're really giving a name to a chunk of
memory somewhere. We'll use C syntax for these examples:

``int i = 5; int j = 6;```

======== =====
location value
======== =====
0x000000 5
0x000001 6
======== =====

This is of course slightly simplified. Anyway, we can introduce indirection by
making a pointer:

``int i = 5; int j = 6; int *pi = &i;```

======== ========
location value
======== ========
0x000000 5
0x000001 6
0x000002 0x000000
======== ========

``pi`` has a pointer to another memory location. We can access the value of the
thing that ``pi`` points at by using the ``*``::

  printf("The value of the thing pi points to is: %d\n", *pi);

The ``*`` de-refereneces the pointer, and gives us the value. Simple!

Here's the issue: you have no idea if the data that is being pointed to is
good. What do you think this code does?::

  int *pi;
  printf("The value of the thing pi points to is: %d\n", *pi);
  
Who knows!?!? Probably something bad, but **certainly** not something expected.
Rust introduces three different kinds of pointers, 'managed,' 'owned,' and
'borrowed.' They indicate different levels of access, so that you know that
different people aren't messing with the things that are being pointed to.
Imagine we spun up ten tasks, passed the same pointer to all of them, and
let them go wild. We'd have no idea what was going on.

Managed Boxes
-------------

Managed boxes are heap allocated and garbage collected. They have a ``@``
prefix. Check it out::

  fn main() {
    let x = @10;
    io::println(int::str(*x));
  }

This prints 10. The ``*`` dereferences, just like in C. The "@10" makes a
managed box pointing at 10.

The big deal is this: managed boxes never leave the task they're made in.
So you can't share them across tasks. This is a nice feature, because the GC
does not need to look at all tasks to determine if ``x`` needs to be collected
or not. So the GC can do fun things like only run when that task is paused for
other reasons, so it's not really 'stopping the world' exactly. You also know
that other people aren't messing with your data, because it **can't** leak
across boundaries.

Owned Boxes
-----------

If you don't want your values to get GC'd, you can use an owned box. This tells
Rust that you own a reference to something, and you'll take care of it
yourself. This is indicated with a ``~``::

  fn main() {
    let x = ~10;
    io::println(int::str(*x));
  }
  
You can't make another pointer to this value::

  fn main() {
    let x = ~10;
    let y = x;
    io::println(int::str(*x));
  }

This yields::

  $ make
  rustc fizzbuzz.rs
  fizzbuzz.rs:3:6: 3:9 warning: unused variable: `y`
  fizzbuzz.rs:3   let y = x;
                      ^~~
  fizzbuzz.rs:4:24: 4:25 error: use of moved variable: `x`
  fizzbuzz.rs:4   io::println(int::str(*x));
                                        ^
  fizzbuzz.rs:3:10: 3:11 note: move of variable occurred here
  fizzbuzz.rs:3   let y = x;
                          ^
  error: aborting due to previous error
  make: *** [build] Error 101

It tells us that we moved the value of ``x`` to ``y`` and points out where
the move happens. Neat. We can make a copy::

  fn main() {
    let x = ~10;
    let y = copy x;
    io::println(int::str(*x));
  }

This will work, though it will tell us that ``y`` was never used. And they
point at two different copies of 10, not the same one.

Because you have the one pointer to a owned box, you can send the pointer to
another task with ``move``::

  use task::spawn;

  fn main() {
    let x = ~10;

    do spawn |move x| {
      io::println(int::str(*x));
    }
  }

If we tried to use ``x`` after the ``move``, it will fail::

  use task::spawn;

  fn main() {
    let x = ~10;

    do spawn |move x| {
      io::println(int::str(*x));
    }

    io::println(int::str(*x));
  }

with::

  $ make
  rustc fizzbuzz.rs
  fizzbuzz.rs:10:24: 10:25 error: use of moved variable: `x`
  fizzbuzz.rs:10   io::println(int::str(*x));
                                         ^
  fizzbuzz.rs:6:5: 8:3 note: move of variable occurred here
  fizzbuzz.rs:6   do spawn |move x| {
  fizzbuzz.rs:7     io::println(int::str(*x));
  fizzbuzz.rs:8   }
  error: aborting due to previous error
  make: *** [build] Error 101

You've already moved it!

Borrowed Pointers
-----------------

Imagine we had this::

  fn plus_one(x: int) -> int {
    x + 1
  }

  fn main() {
    let x = @10;

    io::println(int::str(plus_one(*x)));
  }

Now, this works just fine. But what if we don't want to copy the value of x
when we call ``plus_one``? We'd want to pass a pointer. Easy enough::

  fn plus_one(x: @int) -> int {
    *x + 1
  }

  fn main() {
    let x = @10;

    io::println(int::str(plus_one(x)));
  }

Seems fine. But what about this?::

  fn plus_one(x: @int) -> int {
    *x + 1
  }

  fn main() {
    let x = @10;
    let y = ~10;

    io::println(int::str(plus_one(x)));
    io::println(int::str(plus_one(y))); // uhhhhhhh
  }

``plus_one`` takes a managed box, but we're giving it a unique box. If we try
to compile this, we get this::

  $ make
  rustc fizzbuzz.rs
  fizzbuzz.rs:10:32: 10:33 error: mismatched types: expected `@int` but found `~<VI1>` (expected @-ptr but found ~-ptr)
  fizzbuzz.rs:10   io::println(int::str(plus_one(y)));
                                                 ^
  error: aborting due to previous error
  make: *** [build] Error 101

Makes sense. Expected ``@-ptr`` but found ``~-ptr``. We could do this::

  fn plus_one_managed(x: @int) -> int {
    *x + 1
  }

  fn plus_one_unique(x: ~int) -> int {
    *x + 1
  }

  fn main() {
    let x = @10;
    let y = ~10;

    io::println(int::str(plus_one_managed(x)));
    io::println(int::str(plus_one_unique(y)));
  }

This is pretty obviously a terrible idea. What we want is to take either kind
of pointer: we don't care about changing ownership. We just want to use the
value for a while.

Enter borrowed pointers::

  fn plus_one(x: &int) -> int {
    *x + 1
  }

  fn main() {
    let x = @10;
    let y = ~10;

    io::println(int::str(plus_one(x)));
    io::println(int::str(plus_one(y)));
  }

Borrowed pointers use an ``&``, as you can see. They don't change any ownership
semantics. They do let you write functions that take either kind of pointer,
without caring about those details.
