{% import 'macros/ork.jinja' as ork with context %}

FizzBuzz
========

Of course, the first thing that your job interview for that cushy new Rust job
will task you with is building FizzBuzz. Let's do it!

We'll be assuming you're using the ``rust`` wrapper program, but if you want
you can copy the ``Makefile`` from the previous chapter and use ``make``;
either work.

If you're not familiar, FizzBuzz is a simple programming problem::

  > "Write a program that prints the numbers from 1 to 100. But for multiples of three print “Fizz” instead of the number and for the multiples of five print “Buzz”. For numbers which are multiples of both three and five print “FizzBuzz”."

This will give us a good excuse to go over some basics of Rust: Looping, tests,
printing to standard output, and a host of other simple things.

First, a test. This will go in fizzbuzz.rs::

  #[test]
  fn test_is_three() {
      if is_three(1) {
          fail!("One is not three");
      }
  }

And run it::

  $ rust test fizzbuzz.rs
  fizzbuzz.rs:3:7: 3:15 error: unresolved name `is_three`.
  fizzbuzz.rs:3     if is_three(1) {
                       ^~~~~~~~
  error: aborting due to previous error

This makes sense: We haven't defined any functions yet. Let's define one::

  fn is_three(num: int) -> bool {
      return true;
  }

  #[test]
  fn test_is_three() {
      if is_three(1) {
          fail!("One is not three");
      }
  }

Okay. Here's some new syntax. The ``num: int`` says that we take one argument,
``num``, and that it's of an integer type. The ``-> bool`` says that we return a
boolean, and the ``return true;``, well, returns true.

You'll also note we have an ``if`` statement. It's pretty close to what you'd
expect, but we have curly braces rather than our friends ``do/end``.

Now that we've got that cleared up, let's run our tests::

  $ rust test fizzbuzz.rs
  fizzbuzz.rs:1:12: 1:16 warning: unused variable: `num`
  fizzbuzz.rs:1 fn is_three(num: int) -> bool {
                            ^~~~

  running 1 test
  rust: task failed at 'One is not three', fizzbuzz.rs:8
  test test_is_three ... FAILED

  failures:
      test_is_three

  result: FAILED. 0 passed; 1 failed; 0 ignored

  rust: task failed at 'Some tests failed', /build/src/rust-0.6/src/libstd/test.rs:104
  rust: domain main @0x85d9c0 root task failed


Rust is kind enough to give us a warning: we never used the ``num`` argument. We
then get our failure, "One is not three", because we returned true. Now that
we have a failing test, let's make it pass::

  fn is_three(num: int) -> bool {
    return false;
  }

  #[test]
  fn test_is_three() {
      if is_three(1) {
          fail!("One is not three");
      }
  }

TDD means do the simplest thing! And run it::

  $ rust test fizzbuzz.rs
  fizzbuzz.rs:1:12: 1:16 warning: unused variable: `num`
  fizzbuzz.rs:1 fn is_three(num: int) -> bool {
                            ^~~~

  running 1 test
  test test_is_three ... ok

  result: ok. 1 passed; 0 failed; 0 ignored

Awesome! We pass! We still have that warning, though... let's write another
test, and see what happens::

  fn is_three(num: int) -> bool {
      return false;
  }

  #[test]
  fn test_is_three_with_not_three() {
      if is_three(1) {
        fail!("One is not three");
      }
  }

  #[test]
  fn test_is_three_with_three() {
    if !is_three(3) {
      fail!(~"Three should be three");
    }
  }

  $ rust test fizzbuzz.rs
  fizzbuzz.rs:1:12: 1:16 warning: unused variable: `num`
  fizzbuzz.rs:1 fn is_three(num: int) -> bool {
                            ^~~~

  running 2 tests
  test test_is_three_with_not_three ... ok
  rust: task failed at 'Three should be three', fizzbuzz.rs:15
  test test_is_three_with_three ... FAILED

  failures:
      test_is_three_with_three

  result: FAILED. 1 passed; 1 failed; 0 ignored

  rust: task failed at 'Some tests failed', /build/src/rust-0.6/src/libstd/test.rs:104
  rust: domain main @0x15109c0 root task failed


Great! It showed that our first test passed, and that our second one failed.
Let's make both tests pass::

  fn is_three(num: int) -> bool {
      if num % 3 == 0 {
          return true;
      } else {
          return false;
      }
  }

  #[test]
  fn test_is_three_with_not_three() {
      if is_three(1) {
          fail!("One is not three");
      }
  }

  #[test]
  fn test_is_three_with_three() {
      if !is_three(3) {
          fail!("Three should be three");
      }
  }

  $ rustc test fizzbuzz.rs

  running 2 tests
  test test_is_three_with_not_three ... ok
  test test_is_three_with_three ... ok

  result: ok. 2 passed; 0 failed; 0 ignored

Awesome! This shows off how elses work, as well. It's probably what you
expected. Go ahead and try to refactor this into a one-liner.

Done? How'd you do? Here's mine::

  fn is_three(num: int) -> bool {
      num % 3 == 0
  }

Wait, whaaaat? Yep, the last thing in a function is a return in Rust, but
there's one wrinkle: note there's no semicolon here. If you had one, you'd
get::

  $ rust test fizzbuzz.rs
  fizzbuzz.rs:1:0: 3:1 error: not all control paths return a value
  fizzbuzz.rs:1 fn is_three(num: int) -> bool {
  fizzbuzz.rs:2     num % 3 == 0;
  fizzbuzz.rs:3 }
  error: aborting due to previous error

Basically, ending an expression in Rust with a semicolon ignores the value of
that expression. Another way to think about it is that the semicolon turns the
expression into a statement, and statements don't have values. This is kinda
weird. I don't know how I feel about it. But it is something you should know
about.

Okay, now try to TDD out an ``is_five`` and ``is_fifteen`` methods.
They should work the same way, but this will let you get practice actually
writing it out. Once you see this, you're ready to advance::

  $ rust test fizzbuzz.rs

  running 6 tests
  test test_is_five_with_not_five ... ok
  test test_is_fifteen_with_fifteen ... ok
  test test_is_three_with_not_three ... ok
  test test_is_five_with_five ... ok
  test test_is_three_with_three ... ok
  test test_is_fifteen_with_not_fifteen ... ok

  result: ok. 6 passed; 0 failed; 0 ignored


Okay! Let's talk about the main program now. We've got the tools to build
FizzBuzz, let's make it work. First thing we need to do is print out all
the numbers from one to 100. It's easy!

::

  fn main() {
      for 100.times {
          println("num");
      }
  }

Step one: print **something** 100 times. If you run this with ``rust run`` or
``make`` (not ``make test``!) you should see ``num`` printed 100 times. Note
that our tests didn't actually run. Not only are they not run, they're actually
not even in the executable::

  $ rustc --test fizzbuzz.rs
  $ nm -C fizzbuzz | grep test
  0000000000403cd0 t test_is_five_with_five::_79fbef3fc431adf6::_00
  0000000000403ac0 t test_is_three_with_three::_79fbef3fc431adf6::_00
  0000000000403c10 t test_is_five_with_not_five::_79fbef3fc431adf6::_00
  0000000000403ee0 t test_is_fifteen_with_fifteen::_79fbef3fc431adf6::_00
  0000000000403a00 t test_is_three_with_not_three::_79fbef3fc431adf6::_00
  0000000000403e20 t test_is_fifteen_with_not_fifteen::_79fbef3fc431adf6::_00
                   U test::test_main_static::_e5d562a4bc8c4dd6::_06
  000000000040fea0 T __test::main::_79fbef3fc431adf6::_00
  0000000000614890 D __test::tests::_7c31a8a9617a6a::_00


  $ rustc fizzbuzz.rs
  $ nm -C fizzbuzz | grep test

  $

Crazy, huh? Rust is smart.

(The ``nm`` program lists all the symbols in a binary (executable or library).
The ``-C`` option is important, it "de-mangles" the symbol names. Rust uses the
same mangling scheme as C++, so it's compatible with all the existing tools.
How it works isn't that important, though. It's cool low-level stuff if you're
into that sort of thing.)

Anywho, where were we? Oh, iteration::

  fn main() {
      for 100.times {
          println("num");
      }
  }

Let's talk about ``for``. ``for`` is actually syntax sugar. Here's the equivalent
without ``for``::

  fn main() {
      100.times({
          println("num");
      });
  }

Note the extra parens. Typing out ``});`` really sucks, and having the ``({`` is
also awkward. Just like Ruby, Rust has special syntax when you're passing a
single closure to a method. Awesome. And it shouldn't surprise Rubyists that you
can pass a closure (read: block) to a method, and have it loop. Let's print
out the numbers now. First step: we need to get the number of the current
iteration. Rubyists will do a double take::

  fn main() {
      for 100.times |num| {
          println("num");
      };
  }

Almost the same syntax, but with the pipes *outside* of the curlies. But, if you
try to run this, you'll get an error::

  $ rust build fizzbuzz.rs
  fizzbuzz.rs:45:12: 47:5 error: mismatched types: expected `&fn()` but found `&fn(<V0>)` (incorrect number of function parameters)
  fizzbuzz.rs:45     for 100.times |num| {
  fizzbuzz.rs:46         println("num");
  fizzbuzz.rs:47     }
  fizzbuzz.rs:45:12: 47:5 error: mismatched types: expected `&fn() -> bool` but found `&fn(<V0>) -> bool` (incorrect number of function parameters)
  fizzbuzz.rs:45     for 100.times |num| {
  fizzbuzz.rs:46         println("num");
  fizzbuzz.rs:47     }
  fizzbuzz.rs:45:12: 47:5 error: Unconstrained region variable #3
  fizzbuzz.rs:45     for 100.times |num| {
  fizzbuzz.rs:46         println("num");
  fizzbuzz.rs:47     }
  error: aborting due to 3 previous errors


The big one is this::

  error: mismatched types: expected `&fn()` but found `&fn(<V0>)` (incorrect number of function parameters)

Expected ``fn()`` but got ``fn(<V0>)``. It wants no parameters, but we gave it
one.  Whoops! These kind of crazy compiler errors are a little hard to read,
especially since we don't get them at all in Ruby. The ``<V0>`` is just rust
trying to tell us that it doesn't quite know what type we want: it's the first
(index 0) inferred type it encountered in the program. There is also ``<VIx>``,
for any ``x``, which meants it thought the inferred type was an integer, and
``<VFx>`` for floats.

Anyway, we need a different function::

  fn main() {
      for [1,2,3].iter().advance |&num| {
          println(num)
      }
  }

Yuck! Unfortunately, Rust 0.7 was released during a "limbo" period between two
styles of iterators. Eventually, the ``for`` won't be the syntax sugar it is
now, it will wrap something called ``external iterators`` (basically an object
,with a ``next`` method) and the code above will become ``for &num in [1, 2,
3]``, or something similar to it (the exact syntax is still undecided). Until
then, we're stuck with the old stuff, and the ugly ``.iter().advance`` hack.

Anyway, the ``[]`` s indicate a 'vector', which is kind of like a Ruby array.
The ampersand before the block argument modifies the declaration somehow.
We're going to skim over that until the next section. But that gives us a
variable, ``num``, within the closure. If we run this, we get another error
message::

  $ rust build fizzbuzz.rs
  fizzbuzz.rs:46:16: 46:19 error: mismatched types: expected `&str` but found `<VI2>` (expected &str but found integral variable)
  fizzbuzz.rs:46         println(num);
                                 ^~~
  error: aborting due to previous error

Mismatched types: expected &str but found integral value. It wants a string,
but we gave it a number. Whoops! Now, there's two ways to fix this. The first
is to use the ``to_str`` function::

  fn main() {
      for [1i,2,3].iter().advance |&num| {
          println(nom.to_str())
      }
  }

Awesome. The ``i`` suffix tells Rust that we want the vector to contains
``int``. Otherwise, it wouldn't know which of the various numeric types
(``uint``, ``float``, ``u32``, etc) to use for the ``to_str`` method. The cool
thing about rust's type system is that you only need to provide the "type hint"
once, rather than on every value. Let's run it::

  $ rust run fizzbuzz.rs
  1
  2
  3

Bam! Whew. We had to fight with the compiler a bit, and the errors weren't
great, but that wasn't too bad. The other way to do it is to use the ``fmt!``
function. At least, it looks like a function to me. Here it is::

  fn main() {
    for [1, 2, 3].each |&num| {
      println(fmt!("%d", num));
    }
  }

``fmt!`` is similar to ``str % arg``, or the ``format`` and ``sprintf``
functions in ``Kernel``: it takes a format string, some arguments, and
makes a string out of them. A cool feature of rust that sets it apart from
C or C++, which also have this, is that the format strings are type-checked at
compile time. No more broken format strings!

Anyway, now we have 1 to 3. We need 1 to 100. Typing out all of that would
suck... what to do? This::

  fn main() {
      for std::int::range(1, 101) |num| {
          println(int::to_str(num));
      }
  }


Okay. Range takes two numbers and makes them into a range, then we iterate over
it. Peachy. The ``int`` part means we're using an integer.

Now we can put the two together::

  fn main() {
      for std::int::range(1, 101) |num| {
          let mut answer = "";

          if is_fifteen(num){
              answer = "FizzBuzz";
          }
          else if is_three(num) {
              answer = "Fizz";
          }
          else if is_five(num) {
              answer = "Buzz";
          }
          else {
              answer = "";
          };
          println(answer)
      }
  }

Uhhhh ``let mut``? ``let`` is the way that we make a local variable. ``mut``
means we plan to mutate that variable: yes, variables are immutable by
default. The ``std::int::range`` is the name of the ``int`` range function in
the standard library. We need to fully qualify it, at least if we don't want
to import it...  more on that later.  When I first wrote this, I wrote this::

  let mut answer = "";

We can shorten this up a bit with this syntax::

  fn main() {
      for int::range(1, 101) |num| {
          let mut answer =
              if is_fifteen(num){
                  "FizzBuzz"
              }
              else if is_three(num) {
                  "Fizz"
              }
              else if is_five(num) {
                  "Buzz"
              }
              else {
                  ""
              };
          println(answer)
      }
  }

We've made the ``if`` assign the value to answer. Note that we had to remove
the semicolons again; that lets the expression give its value to ``answer.`` Note
that this _also_ makes answer immutable, so we can remove the ``mut``::

  fn main() {
      for int::range(1, 101) |num| {
          let answer =
              if is_fifteen(num){
                  "FizzBuzz"
              }
              else if is_three(num) {
                  "Fizz"
              }
              else if is_five(num) {
                  "Buzz"
              }
              else {
                  ""
              };
          println(answer)
      }
  }

Not too shabby! I love eliminating mutable state.

Of course, this version gives us lots of empty lines, so what we actually want
is::

  fn main() {
      for std::int::range(1, 101) |num| {
          let answer =
              if is_fifteen(num){
                  ~"FizzBuzz"
              }
              else if is_three(num) {
                  ~"Fizz"
              }
              else if is_five(num) {
                  ~"Buzz"
              }
              else {
                  num.to_str()
              };
          println(answer)
      }
  }

What's up with the tildes? Like the ampersand, they modify the declaration
somehow. I added it because running without it gives an error message that
implies you need it: give it a shot. Because our variables are typed, we have
to convert the number in the ``else`` case to a string. In Ruby we'd just let
it be a ``Fixnum`` if it was a number. Oh well.

Because the ``if`` returns a value, we could also do something like this::

  fn main() {
      for std::int::range(1, 101) |num| {
          println(
            if is_fifteen(num) { ~"FizzBuzz" }
            else if is_three(num) { ~"Fizz" }
            else if is_five(num) { ~"Buzz" }
            else { num.to_str() }
          );
      }
  }

It's more compact, and removes the intermediate variable all together.

We can do one other thing too: this whole ``if/fail!`` thing so common in tests
seems too complex.  Why do we have to write if over and over and over again?
Meet ``assert!``::

  #[test]
  fn test_is_fifteen_with_fifteen() {
    assert!(is_fifteen(15))
  }

This will fail if it gets false, and pass if it gets true. Simple! You can also
give it a message to be printed when the assertion fails, mostly useful when you
are using ``assert!`` to test for preconditions and such::

  fn main() {
    assert!(1 == 0, "1 does not equal 0!");
  }

Try running it.

Anyway, awesome! We've conquered FizzBuzz. ``is_fifteen`` isn't the best named
method, but we're just learning. ;) Here's my full final code::

  fn is_three(num: int) -> bool {
      num % 3 == 0
  }

  #[test]
  fn test_is_three_with_not_three() {
      assert!(!is_three(1))
  }

  #[test]
  fn test_is_three_with_three() {
      assert!(is_three(3))
  }

  fn is_five(num: int) -> bool {
      num % 5 == 0
  }

  #[test]
  fn test_is_five_with_not_five() {
      assert!(!is_five(1))
  }

  #[test]
  fn test_is_five_with_five() {
      assert!(is_five(5))
  }

  fn is_fifteen(num: int) -> bool {
      num % 15 == 0
  }

  #[test]
  fn test_is_fifteen_with_not_fifteen() {
    assert!(!is_fifteen(1))
  }

  #[test]
  fn test_is_fifteen_with_fifteen() {
      assert!(is_fifteen(15))
  }

  fn main() {
      for std::int::range(1, 101) |num| {
          println(
              if is_fifteen(num) { ~"FizzBuzz" }
              else if is_three(num) { ~"Fizz" }
              else if is_five(num) { ~"Buzz" }
              else { num.to_str() }
          );
      }
  }
