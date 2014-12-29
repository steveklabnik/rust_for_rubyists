Pointers, and ownership, oh my!
======================================

Since you program in Ruby, you probably don't know about pointers, nor care. If
you're going to work in a language like Rust, though, you gotta know about
them. So let's talk about the concept real quick, then discuss how Rust handles
pointers.

Pointer recap
-------------

When you create a variable, you're really giving a name to a chunk of
memory somewhere. We'll use C syntax for these examples:

~~~ {.c}
int i = 5; int j = 6;
~~~

      location   value
      ---------- -------
      0x000000   5
      0x000001   6

This is of course slightly simplified. Anyway, we can introduce
indirection by making a pointer:

~~~ {.c}
int i = 5; int j = 6; int *pi = &i;
~~~

      location   value
      ---------- ----------
      0x000000   5
      0x000001   6
      0x000002   0x000000

`pi` has a pointer to another memory location. We can access the value
of the thing that `pi` points at by using the `*`:

~~~ {.c}
printf("The value of the thing pi points to is: %d\n", *pi);
~~~

The `*` dereferences the pointer, and gives us the value. Simple!

Here's the issue: you have no idea if the data that is being pointed to
is good. What do you think this code does?:

~~~ {.c}
int *pi;
printf("The value of the thing pi points to is: %d\n", *pi);
~~~

Who knows!?!? Probably something bad, but **certainly** not something expected.
Rust introduces two different kinds of pointers: 'owned' and 'borrowed.' They
indicate different levels of access, so that you know that different people
aren't messing with the things that are being pointed to.  Imagine we spun up
ten tasks, passed the same pointer to all of them, and let them go wild. We'd
have no idea what was going on.

Owned Pointer
-------------

An owned pointer tells Rust that you own a reference to something. We can
create one with the `box` keyword:

~~~ {.rust}
fn main() {
    let x = box 10i;
    println!("{}", *x);
}
~~~

You can't make another owned pointer to this value:

~~~ {.rust}
fn main() {
    let x = box 10i;
    let y = x;
    println!("{}", *x);
}
~~~

This yields:

    $ rustc owned.rs && ./owned
    owned.rs:4:22: 4:24 error: use of moved value: `*x`
    owned.rs:4     println!("{}", *x);
                                  ^~
    note: in expansion of format_args!
    <std macros>:2:23: 2:77 note: expansion site
    <std macros>:1:1: 3:2 note: in expansion of println!
    owned.rs:4:5: 4:26 note: expansion site
    owned.rs:3:9: 3:10 note: `x` moved here because it has type `Box<int>`, which is moved by default 
    owned.rs:3     let y = x;
                       ^
    owned.rs:3:9: 3:10 help: use `ref` to override
    owned.rs:3     let y = x;
                       ^
    error: aborting due to previous error

It tells us that we moved the value of `x` to `y` and points out where
the move happens. Neat. We can make a copy:

~~~ {.rust}
fn main() {
    let x = box 10i;
    let y = x.clone();
    println!("{}", *x);
}
~~~

This will work, though it will tell us that `y` was never used. And they
point at two different copies of 10, not the same one.

That said, you generally don't need to use an owned pointer. You generally need
them for recursive data structures, or when you have a _huge_ chunk of data
that you're passing around between many functions.

Instead, use a borrowed pointer.

Borrowed Pointers
-----------------

Enter borrowed pointers:

~~~ {.rust}
fn plus_one(x: &int) -> int {
    *x + 1
}

fn main() {
    let y = box 10i;

    println!("{}", plus_one(&*y));
}
~~~

Borrowed pointers use an `&`, as you can see. They don't change any
ownership semantics. They do let you write functions that take any other
kind of pointer, without caring about those details. The compiler makes
sure that all borrowed pointers do not outlive the thing they point to,
which means you don't have to worry about use-after-free or any of the
other hairy pointer issues in C.

Borrowed pointers can get a lot more complex, but this is the gist of
them. Always use borrowed pointers when you can, they should be your
go-to solution for all your pointer needs.

Pointer strategy
----------------

Basically, idiomatic Rust code will... not use pointers at all, and just use
values. If you're writing a function that needs to take a pointer, use borrowed
pointers, rather than being specific.

There are some more complex heap-allocated types in Rust, but they're outside
the scope of this introduction.
