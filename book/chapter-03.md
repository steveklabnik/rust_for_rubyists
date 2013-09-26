Writing Your First Rust Program
===============================

Okay! Let's get down to it: in order to call yourself an "X Programmer,"
you must write "Hello, world" in X. So let's do it. Open up a text file:
I'll use `vim` because I'm that kind of guy, but use whatever you want.
Rust programs end in `.rs`:

    $ vim hello.rs

Put this in it:

~~~ {.rust}
    fn main() {
        println("Hello, world.");
    }
~~~

And compile it with `rustc`:

    $ rustc hello.rs

It should compile without error. If you get one, double check that you
have the semicolons, the curlies, and the parentheses. Errors look like
this:

    $ rustc hello.rs
    hello.rs:2:4: 2:11 error: expected `{` but found `println`
    hello.rs:2     println("Hello, world.");
                   ^~~~~~~

This happened when I left off the curly brace after the `main` function
above. To run your program, do the Usual UNIX Thing:

    $ ./hello

And you should see "Hello, world." print to the screen. Congrats!

There's an easier way to do this, too. Rust provides another tool, `rust`,
which wraps up a lot of functionality. We won't use `rustc` directly for the
rest of this book, because you almost always want to be working with the
higher-level tooling. Check it:

    $ rust run hello.rs
    Hello, world!

Yup, `rust run` combines the compile and run step. This will probably feel more
normal to you than keeping the two steps apart. It's only good for simple things,
though: soon enough we'll be using `rustpkg` instead. But that's for later...
