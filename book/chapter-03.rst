{% import 'macros/ork.jinja' as ork with context %}

Writing Your First Rust Program
===============================

Okay! Let's get down to it: in order to call yourself an "X Programmer," you
must write "Hello, world" in X. So let's do it. Open up a text file: I'll use
``vim`` because I'm that kind of guy, but use whatever you want. Rust programs
end in ``.rs``:

::

  $ vim hello.rs


Put this in it::

  use io::println;

  fn main() {
      println("Hello, world.");
  }


And compile it with ``rustc``::

  $ rustc hello.rs

It should compile without error. If you get one, double check that you have the
semicolons, the curlies, and the parentheses. Errors look like this::

  $ rustc hello.rs
  hello.rs:3:0: 3:2 error: expected `;` but found `fn`
  hello.rs:3 fn main() {
             ^~

This happened when I left off the semicolon after the ``use`` statement above.
To run your program, do the Usual UNIX Thing::

  $ ./hello

And you should see "Hello, world." print to the screen. Congrats!
