Boxes, Pointers, and ownership, oh my!
======================================

Since you program in Ruby, you probably don't know about pointers, nor
care. If you're going to work in a language like Rust, though, you gotta
know about them. So let's talk about the concept real quick, then
discuss how Rust handles pointers.

Pointer recap
-------------

When you create a variable, you're really giving a name to a chunk of
memory somewhere. We'll use C syntax for these examples:

`int i = 5; int j = 6;`

  location   value
  ---------- -------
  0x000000   5
  0x000001   6

This is of course slightly simplified. Anyway, we can introduce
indirection by making a pointer:

`int i = 5; int j = 6; int *pi = &i;`

  location   value
  ---------- ----------
  0x000000   5
  0x000001   6
  0x000002   0x000000

`pi` has a pointer to another memory location. We can access the value
of the thing that `pi` points at by using the `*`:

    printf("The value of the thing pi points to is: %d\n", *pi);

The `*` dereferences the pointer, and gives us the value. Simple!

Here's the issue: you have no idea if the data that is being pointed to
is good. What do you think this code does?:

    int *pi;
    printf("The value of the thing pi points to is: %d\n", *pi);

Who knows!?!? Probably something bad, but **certainly** not something
expected. Rust introduces three different kinds of pointers, 'managed,'
'owned,' and 'borrowed.' They indicate different levels of access, so
that you know that different people aren't messing with the things that
are being pointed to. Imagine we spun up ten tasks, passed the same
pointer to all of them, and let them go wild. We'd have no idea what was
going on.

Managed Boxes
-------------

Managed boxes are heap allocated and garbage collected. They are the
most similar to Ruby object references. They have a `@` prefix. Check it
out:

    fn main() {
        let x = @10;
        println((*x).to_str());
    }

This prints 10. The `*` dereferences, just like in C. The
["@10](mailto:"@10)" makes a managed box pointing at 10.

The big deal is this: managed boxes never leave the task they're made
in. So you can't share them across tasks. This is a nice feature,
because the GC does not need to look at all tasks to determine if `x`
needs to be collected or not. So the GC can do fun things like only run
when that task is paused for other reasons, so it's not really 'stopping
the world' exactly. You also know that other people aren't messing with
your data, because it **can't** leak across boundaries.

You should almost never used managed boxes. Only resort to them when you
cannot use another type of pointer. They are (relatively) inefficient
and, when using `@mut`, can cause dynamic, runtime failure!

Owned Boxes
-----------

If you don't want your values to get GC'd, you can use an owned box.
This tells Rust that you own a reference to something, and you'll take
care of it yourself. This is indicated with a `~`:

    fn main() {
        let x = ~10;
        println((*x).to_str());
    }

You can't make another owned pointer to this value:

    fn main() {
        let x = ~10;
        let y = x;
        println((*x).to_str());
    }

This yields:

    $ rust run owned.rs
    owned.rs:3:8: 3:11 warning: unused variable: `y`
    owned.rs:3     let y = x;
                       ^~~
    owned.rs:4:25: 4:26 error: use of moved value: `x`
    owned.rs:4     println((*x).to_str());
                                        ^
    owned.rs:3:12: 3:13 note: `x` moved here because it has type ~int, which is moved by default (use `copy` to override)
    owned.rs:3     let y = x;
                           ^
    error: aborting due to previous error

It tells us that we moved the value of `x` to `y` and points out where
the move happens. Neat. We can make a copy:

    fn main() {
        let x = ~10;
        let y = x.clonse();
        println((*x).to_str());
    }

This will work, though it will tell us that `y` was never used. And they
point at two different copies of 10, not the same one.

Genearly, you should use owned boxes where possible, rather than managed
boxes. They are much more efficient (no GC!), because they are analyzed
at compile-time to make sure they are used correctly. Some things just
aren't possible without managed boxes, though. Frequently, though, you
don't even need to use owned boxes, because Rust has...

Borrowed Pointers
-----------------

Imagine we had this:

    fn plus_one(x: int) -> int {
        x + 1
    }

    fn main() {
        let x = ~10;

        println(plus_one(*x).to_str());
    }

Now, this works just fine. But what if we don't want to copy the value
of x when we call `plus_one`? We'd want to pass a pointer. Easy enough:

    fn plus_one(x: ~int) -> int {
        *x + 1
    }

    fn main() {
      let x = ~10;

      println(plus_one(x).to_str());
    }

Seems fine. But what about this?:

    fn plus_one(x: ~int) -> int {
        *x + 1
    }

    fn main() {
        let x = ~10;
        let y = @10;

        println(plus_one(x).to_str());
        println(plus_one(y).to_str()); // uhhhhhhh
    }

`plus_one` takes an owned box, but we're giving it a managed box. If we
try to compile this, we get this:

    $ rust run owned.rs
    owned.rs:9:33: 9:34 error: mismatched types: expected `~int` but found `@<VI1>` (expected ~-ptr but found @-ptr)
    owned.rs:9     println(plus_one(y).to_str());
                                    ^
    error: aborting due to previous error

Makes sense. Expected `~-ptr` but found `@-ptr`. We could do this:

    fn plus_one_managed(x: @int) -> int {
        *x + 1
    }

    fn plus_one_owned(x: ~int) -> int {
        *x + 1
    }

    fn main() {
        let x = @10;
        let y = ~10;

        println(plus_one_managed(x).to_str());
        println(plus_one_owned(y).to_str());
    }

This is pretty obviously a terrible idea. What we want is to take either
kind of pointer: we don't care about changing ownership. We just want to
use the value for a while.

Enter borrowed pointers:

    fn plus_one(x: &int) -> int {
        *x + 1
    }

    fn main() {
        let x = @10;
        let y = ~10;

        println(plus_one(x).to_owned());
        println(plus_one(y).to_owned());
    }

Borrowed pointers use an `&`, as you can see. They don't change any
ownership semantics. They do let you write functions that take either
kind of pointer, without caring about those details. The compiler makes
sure that all borrowed pointers do not outlive the thing they point to,
which means you don't have to worry about use-after-free or any of the
other hairy pointer issues in C.

Borrowed pointers can get a lot more complex, but this is the gist of
them. Always use borrowed pointers when you can, they should be your
go-to solution for all your pointer needs.
