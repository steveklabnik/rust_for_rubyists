{% import 'macros/ork.jinja' as ork with context %}

FizzBuzz
========

Of course, the first thing that your job interview for that cushy new Rust job
will task you with is building FizzBuzz. Let's do it!

If you haven't done the previous chapter's worth of setup, please copy the
Makefile we built into your current directory: we'll be assuming you've set
that up in this chapter.

If you're not familiar, FizzBuzz is a simple programming problem::

  > "Write a program that prints the numbers from 1 to 100. But for multiples of three print “Fizz” instead of the number and for the multiples of five print “Buzz”. For numbers which are multiples of both three and five print “FizzBuzz”."

This will give us a good excuse to go over some basics of Rust: Looping, tests,
printing to standard output, and a host of other simple things.

First, a test. This will go in fizzbuzz.rs::

  extern mod std;

  #[test]
  fn test_is_three() {
    if is_three(1) {
      fail ~"One is not three";
    }
  }

  fn main() {
  }

And run it::

  $ make test
  rustc fizzbuzz.rs --test
  fizzbuzz.rs:5:5: 5:13 error: unresolved name: is_three
  fizzbuzz.rs:5   if is_three(1) {
                     ^~~~~~~~
  error: aborting due to previous error
  make: *** [build-test] Error 101


This makes sense: We haven't defined any functions yet. Let's define one::

  extern mod std;

  fn is_three(num: int) -> bool {
    return true;
  }

  #[test]
  fn test_is_three() {
    if is_three(1) {
      fail ~"One is not three";
    }
  }

  fn main() {
  }

Okay. Here's some new syntax. The ``num: int`` says that we take one argument,
``num``, and that it's of an integer type. The ``-> bool`` says that we return a
boolean, and the ``return true;``, well, returns true.

You'll also note we have an ``if`` statement. It's pretty close to what you'd
expect, but we have curly braces rather than our friends ``do/end``.

Now that we've got that cleared up, let's run our tests::

  $ make test
  rustc fizzbuzz.rs --test
  fizzbuzz.rs:3:12: 3:16 warning: unused variable: `num`
  fizzbuzz.rs:3 fn is_three(num: int) -> bool {
                            ^~~~
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz

  running 1 test
  rust: task failed at 'One is not three', fizzbuzz.rs:10
  test test_is_three ... FAILED

  failures:
      test_is_three

  result: FAILED. 0 passed; 1 failed; 0 ignored

  rust: task failed at 'Some tests failed', /private/tmp/rust-w7Y4/rust-0.5/src/libstd/test.rs:62
  rust: domain main @0x7fbd04800010 root task failed
  rust: task failed at 'killed', /private/tmp/rust-w7Y4/rust-0.5/src/libcore/task/mod.rs:570
  make: *** [run] Error 101

Rust is kind enough to give us a warning: we never used the ``num`` argument. We
then get our failure, "One is not three", because we returned true. Now that
we have a failing test, let's make it pass::

  extern mod std;

  fn is_three(num: int) -> bool {
    return false;
  }

  #[test]
  fn test_is_three() {
    if is_three(1) {
      fail ~"One is not three";
    }
  }

  fn main() {
  }

TDD means do the simplest thing! And run it::

  $ make test
  rustc fizzbuzz.rs --test
  fizzbuzz.rs:3:12: 3:16 warning: unused variable: `num`
  fizzbuzz.rs:3 fn is_three(num: int) -> bool {
                            ^~~~
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz

  running 1 test
  test test_is_three ... ok

  result: ok. 1 passed; 0 failed; 0 ignored

Awesome! We pass! We still have that warning, though... let's write another
test, and see what happens::

  extern mod std;

  fn is_three(num: int) -> bool {
    return false;
  }

  #[test]
  fn test_is_three_with_not_three() {
    if is_three(1) {
      fail ~"One is not three";
    }
  }

  #[test]
  fn test_is_three_with_three() {
    if !is_three(3) {
      fail ~"Three should be three";
    }
  }

  fn main() {
  }

  $ make test
  rustc fizzbuzz.rs --test
  fizzbuzz.rs:3:12: 3:16 warning: unused variable: `num`
  fizzbuzz.rs:3 fn is_three(num: int) -> bool {
                            ^~~~
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz

  running 2 tests
  rust: task failed at 'Three should be three', fizzbuzz.rs:17
  test test_is_three_with_not_three ... ok
  test test_is_three_with_three ... FAILED

  failures:
      test_is_three_with_three

  result: FAILED. 1 passed; 1 failed; 0 ignored

  rust: task failed at 'Some tests failed', /private/tmp/rust-w7Y4/rust-0.5/src/libstd/test.rs:62
  rust: domain main @0x7fe21b008c10 root task failed
  rust: task failed at 'killed', /private/tmp/rust-w7Y4/rust-0.5/src/libcore/task/mod.rs:570
  make: *** [run] Error 101

Great! It showed that our first test passed, and that our second one failed.
Let's make both tests pass::

  extern mod std;

  fn is_three(num: int) -> bool {
    if num % 3 == 0 {
      return true;
    }
    else {
      return false;
    }
  }

  #[test]
  fn test_is_three_with_not_three() {
    if is_three(1) {
      fail ~"One is not three";
    }
  }

  #[test]
  fn test_is_three_with_three() {
    if !is_three(3) {
      fail ~"Three should be three";
    }
  }

  fn main() {
  }

  $ make test
  rustc fizzbuzz.rs --test
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz

  running 2 tests
  test test_is_three_with_three ... ok
  test test_is_three_with_not_three ... ok

  result: ok. 2 passed; 0 failed; 0 ignored

Awesome! This shows off how elses work, as well. It's probably what you expected. Go ahead and try to refactor this into a one-liner.

Done? How'd you do? Here's mine::

  fn is_three(num: int) -> bool {
    num % 3 == 0
  }

Wait, whaaaat? Yep, the last thing in a function is a return in Rust, but
there's one wrinkle: note there's no semicolon here. If you had one, you'd
get::

  $ make test
  rustc fizzbuzz.rs --test
  fizzbuzz.rs:3:0: 5:1 error: not all control paths return a value
  fizzbuzz.rs:3 fn is_three(num: int) -> bool {
  fizzbuzz.rs:4   num % 3 == 0;
  fizzbuzz.rs:5 }
  error: aborting due to 1 previous error
  make: *** [build-test] Error 101

Basically, ending an expression in Rust with a semicolon ignores the value of
that expression. This is kinda weird. I don't know how I feel about it. But it
is something you should know about.

Okay, now try to TDD out an ``is_five`` and ``is_fifteen`` methods.
They should work the same way, but this will let you get practice actually
writing it out. Once you see this, you're ready to advance::

  $ make test
  rustc fizzbuzz.rs --test
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz

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
      io::println("num");
    }
  }

Step one: print **something** 100 times. If you run this with ``make`` (not ``make
test``!) you should see ``num`` printed 100 times. Note that our tests didn't
actually run. Not only are they not run, they're actually not even in
the executable::

  $ rustc fizzbuzz.rs --test
  warning: no debug symbols in executable (-arch x86_64)

  $ nm fizzbuzz | grep test
  00000001000014a0 t __ZN22test_is_five_with_five16_9f1419ad40c33243_00E
  0000000100001170 t __ZN24test_is_three_with_three16_9f1419ad40c33243_00E
  0000000100001340 t __ZN26test_is_five_with_not_five16_9f1419ad40c33243_00E
  00000001000017d0 t __ZN28test_is_fifteen_with_fifteen16_9f1419ad40c33243_00E
  0000000100000e60 t __ZN28test_is_three_with_not_three16_9f1419ad40c33243_00E
  0000000100001660 t __ZN32test_is_fifteen_with_not_fifteen16_9f1419ad40c33243_00E
                   U __ZN4test9test_main16_d49dbca63e2e5743_05E
  0000000100001950 T __ZN6__test4main16_9f1419ad40c33243_00E
  0000000100001c30 T __ZN6__test5tests16_fea9bebe46b6e9c3_00E
  0000000100003150 t __ZN6__test5tests4anon12expr_fn_2901E
  0000000100003180 t __ZN6__test5tests4anon12expr_fn_2905E
  00000001000031b0 t __ZN6__test5tests4anon12expr_fn_2909E
  00000001000031e0 t __ZN6__test5tests4anon12expr_fn_2913E
  0000000100003210 t __ZN6__test5tests4anon12expr_fn_2917E
  0000000100003240 t __ZN6__test5tests4anon12expr_fn_2921E

  $ rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)

  $ nm fizzbuzz | grep test

  $

Crazy, huh? Rust is smart.

Anywho, where were we? Oh, iteration::

  fn main() {
    for 100.times {
      io::println("num");
    }
  }

Let's talk about ``for``. ``for`` is actually syntax sugar. Here's the equivalent
without ``for``::

  fn main() {
    100.times({
      io::println("num");
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
      io::println("num");
    };
  }

Almost the same syntax, but with the pipes *outside* of the curlies. But, if you
try to run this, you'll get an error::

  $ make
  rustc fizzbuzz.rs
  fizzbuzz.rs:58:10: 61:3 error: mismatched types: expected `&fn()` but found `&fn(<V0>)` (incorrect number of function parameters)
  fizzbuzz.rs:58   for 100.times |num| {
  fizzbuzz.rs:59     //io::println(int::str(num))
  fizzbuzz.rs:60     io::println("num")
  fizzbuzz.rs:61   }
  fizzbuzz.rs:58:10: 61:3 error: mismatched types: expected `&fn() -> bool` but found `&fn(<V0>) -> bool` (incorrect number of function parameters)
  fizzbuzz.rs:58   for 100.times |num| {
  fizzbuzz.rs:59     //io::println(int::str(num))
  fizzbuzz.rs:60     io::println("num")
  fizzbuzz.rs:61   }
  fizzbuzz.rs:58:10: 61:3 error: Unconstrained region variable #12
  fizzbuzz.rs:58   for 100.times |num| {
  fizzbuzz.rs:59     //io::println(int::str(num))
  fizzbuzz.rs:60     io::println("num")
  fizzbuzz.rs:61   }
  error: aborting due to 3 previous errors
  make: *** [build] Error 101

The big one is this::

  error: mismatched types: expected `&fn()` but found `&fn(<V0>)` (incorrect number of function parameters)

Expected ``fn()`` but got ``fn(<V0>)``. It wants no parameters, but we gave it one.
Whoops! These kind of crazy compiler errors are a little hard to read,
especially since we don't get them at all in Ruby.

Anyway, we need a different function::

  fn main() {
    for [1,2,3].each |&num| {
      io::println(num)
    }
  }

Okay. The ``[]`` s indicate a 'vector', which is kind of like a Ruby array. The
ampersand before the block argument is sort of like the tilde before that
string we found before: it modifies the declaration somehow. We're going to
skim over that until the next section. But that gives us a variable, ``num``,
within the closure. If we run this, we get another error message::

  $ make
  rustc fizzbuzz.rs
  fizzbuzz.rs:60:16: 60:19 error: mismatched types: expected `&/str` but found `<VI2>` (expected &/str but found integral variable)
  fizzbuzz.rs:60     io::println(num)
                                 ^~~
  error: aborting due to previous error
  make: *** [build] Error 101

Mismatched types: expected &/str but found integral value. It wants a string,
but we gave it a number. Whoops! Let's coerce it::

  fn main() {
    for [1,2,3].each |&num| {
      io::println(int::str(num))
    }
  }

Awesome. Those double colons are just like Ruby: namespacing. The io namespace
has a println function, the int namespace has a str function. This should
compile and give you output::

  $ make
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz
  1
  2
  3

Bam! Whew. We had to fight with the compiler a bit, and the errors weren't
great, but that wasn't too bad.

What I *will* tell you is that this took me *forever* to figure out. The
documentation for ``each`` says this::

  Method each

  fn each(blk: &fn(v: &A) -> bool)

That's it. See yourself: http://static.rust-lang.org/doc/0.5/core/iter.html

What's worse is that each _used_ to have a different signature, and not return
a boolean. So all the examples I could find were just wrong. Rust has changed
a lot from 0.1 to 0.5, and so if you don't have an example for the right
version of Rust, it may just plain not compile. It's very frustrating. That's
why you're reading this book!

Anyway, now we have 1 to 3. We need 1 to 100. Typing out all of that would
suck... what to do? This::

  fn main() {
    for int::range(1, 101) |num| {
      io::println(int::str(num));
    }
  }


Okay. Range takes two numbers and makes them into a range, then we iterate over
it. Peachy. The ``uint`` part means we're using an unsigned integer, which makes
sense: We're not doing anything that's negative.

Now we can put the two together::

  fn main() {
    for int::range(1, 101) |num| {
      let mut answer;
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
      io::println(answer)
    }
  }

Uhhhh ``let mut``? ``let`` is the way that we make a local variable. ``mut`` means
we plan to mutate that variable: yes, variables are immutable by default.
When I first wrote this, I wrote this::

  let mut answer = "";

And when I compiled, Rust gave me this warning::

  fizzbuzz.rs:59:12: 59:20 warning: value assigned to `answer` is never read
  fizzbuzz.rs:59     let mut answer = "";
                             ^~~~~~~~

Neat! We never use that default, so might as well not set it. Rust knows that
we never read it due to crazy magic stuff that I don't fully understand yet
called 'region analysis.'

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
      io::println(answer)
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
      io::println(answer)
    }
  }

Not too shabby! I love eliminating mutable state.

Of course, this version gives us lots of empty lines, so what we actually want
is::

  fn main() {
    for int::range(1, 101) |num| {
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
          int::str(num)
        };
      io::println(answer)
    }
  }

Remember that the tilde has an effect that we haven't talked about yet. I added
it because running without it gives an error message that implies you need it:
give it a shot. Because our variables are typed, we have to coerce the number
in the ``else`` case to a string. In Ruby we'd just let it be a ``Fixnum`` if
it was a number. Oh well.

Because the ``if`` returns a value, we could also do something like this::

  fn main() {
    for int::range(1, 101) |num| {
      io::println(
        if is_fifteen(num) { ~"FizzBuzz" }
        else if is_three(num) { ~"Fizz" }
        else if is_five(num) { ~"Buzz" }
        else { int::str(num) }
      );
    }
  }

It's more compact, and removes the intermediate variable all together.

We can do one other thing too: this whole ``if/fail`` thing seems too complex.
Why do we have to write if over and over and over again? Meet ``assert``::

  #[test]
  fn test_is_fifteen_with_fifteen() {
    assert is_fifteen(15)
  }

This will fail if it gets false, and pass if it gets true. Simple!

Anyway, awesome! We've conquered FizzBuzz. ``is_fifteen`` isn't the best named
method, but we're just learning. ;) Here's my full final code::

  extern mod std;

  fn is_three(num: int) -> bool {
    num % 3 == 0
  }

  #[test]
  fn test_is_three_with_not_three() {
    assert !is_three(1)
  }

  #[test]
  fn test_is_three_with_three() {
    assert is_three(3)
  }

  fn is_five(num: int) -> bool {
    num % 5 == 0
  }

  #[test]
  fn test_is_five_with_not_five() {
    assert !is_five(1)
  }

  #[test]
  fn test_is_five_with_five() {
    assert is_five(5)
  }

  fn is_fifteen(num: int) -> bool {
    num % 15 == 0
  }

  #[test]
  fn test_is_fifteen_with_not_fifteen() {
    assert !is_fifteen(1)
  }

  #[test]
  fn test_is_fifteen_with_fifteen() {
    assert is_fifteen(15)
  }


  fn main() {
    for int::range(1, 101) |num| {
      io::println(
        if is_fifteen(num) { ~"FizzBuzz" }
        else if is_three(num) { ~"Fizz" }
        else if is_five(num) { ~"Buzz" }
        else { int::str(num) }
      );
    }
  }
