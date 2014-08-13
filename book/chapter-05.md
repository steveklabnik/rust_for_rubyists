FizzBuzz
========

Of course, the first thing that your job interview for that cushy new
Rust job will task you with is building FizzBuzz. Let's do it!

If you're not familiar, FizzBuzz is a simple programming problem:

> "Write a program that prints the numbers from 1 to 100. But for multiples of
> three print “Fizz” instead of the number and for the multiples of five print
> “Buzz”. For numbers which are multiples of both three and five print
> “FizzBuzz”."

This will give us a good excuse to go over some basics of Rust: Looping,
tests, printing to standard output, and a host of other simple things.

First, a test. This will go in fizzbuzz.rs:

~~~ {.rust}
#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        fail!("One is not three");
    }
}
~~~

And compile it:

    $ rustc --test fizzbuzz.rs
    fizzbuzz.rs:3:8: 3:20 error: unresolved name `div_by_three`.
    fizzbuzz.rs:3     if div_by_three(1) {
                         ^~~~~~~~~~~~
    error: aborting due to previous error


This makes sense: We haven't defined any functions yet. Let's define
one:

~~~ {.rust}
fn div_by_three(num: int) -> bool {
   true
}

#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        fail!("One is not three");
    }
}
~~~

Okay. Here's some new syntax. The `num: int` says that we take one
argument, `num`, and that it's of an integer type. The `-> bool` says
that we return a boolean, and the `true`, well, returns true. Just like
Ruby, the value of the last expression gets returned.

You'll also note we have an `if` expression. It's pretty close to what
you'd expect, but we have curly braces rather than our friends `do/end`.

Now that we've got that cleared up, let's compile and run our tests:

    $ rustc --test fizzbuzz.rs
    fizzbuzz.rs:1:17: 1:18 warning: unused variable: `num`, #[warn(unused_variable)] on by default
    fizzbuzz.rs:1 fn div_by_three(num: int) -> bool {
                                  ^

    $ ./fizzbuzz
    running 1 test
    test test_div_by_three ... FAILED

    failures:

    ---- test_div_by_three stdout ----
        task 'test_div_by_three' failed at 'One is not three', fizzbuzz.rs:8



    failures:
        test_div_by_three

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

    task '<main>' failed at 'Some tests failed', /home/steve/src/rust/src/libtest/lib.rs:243


Rust is kind enough to give us a warning: we never used the `num`
argument. We then get our failure, "One is not three", because we
returned true. Now that we have a failing test, let's make it pass:

~~~ {.rust}
fn div_by_three(num: int) -> bool {
   false
}

#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        fail!("One is not three");
    }
}
~~~

TDD means do the simplest thing! Compile and run it:

    $ rustc --test fizzbuzz.rs
    fizzbuzz.rs:1:17: 1:18 warning: unused variable: `num`, #[warn(unused_variable)] on by default
    fizzbuzz.rs:1 fn div_by_three(num: int) -> bool {
                                  ^

    $ ./fizzbuzz
    running 1 test
    test test_div_by_three ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

Awesome! We pass! We still have that warning, though... let's write
another test, and see what happens:

~~~ {.rust}
fn div_by_three(num: int) -> bool {
   false
}

#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        fail!("One is not three");
    }
}

#[test]
fn test_div_by_three_with_three() {
    if !div_by_three(3) {
        fail!("Three should be three");
    }
}
~~~

    $ rustc --test fizzbuzz.rs
    fizzbuzz.rs:1:17: 1:18 warning: unused variable: `num`, #[warn(unused_variable)] on by default
    fizzbuzz.rs:1 fn div_by_three(num: int) -> bool {
                                  ^


    $ ./fizzbuzz
    running 2 tests
    test test_div_by_three ... ok
    test test_div_by_three_with_three ... FAILED

    failures:

    ---- test_div_by_three_with_three stdout ----
        task 'test_div_by_three_with_three' failed at 'Three should be three', fizzbuzz.rs:15



    failures:
        test_div_by_three_with_three

    test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured

    task '<main>' failed at 'Some tests failed', /home/steve/src/rust/src/libtest/lib.rs:243


Great! It showed that our first test passed, and that our second one
failed. Let's make both tests pass:

~~~ {.rust}
fn div_by_three(num: int) -> bool {
    if num % 3 == 0 {
        true
    } else {
        false
    }
}

#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        fail!("One is not three");
    }
}

#[test]
fn test_div_by_three_with_three() {
    if !div_by_three(3) {
        fail!("Three should be three");
    }
}
~~~

    $ rustc --test fizzbuzz.rs && ./fizzbuzz
    running 2 tests
    test test_div_by_three_with_three ... ok
    test test_div_by_three ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured


Awesome! This shows off how elses work, as well. It's probably what you
expected. Go ahead and try to refactor this into a one-liner.

Done? How'd you do? Here's mine:

~~~ {.rust}
fn div_by_three(num: int) -> bool {
    num % 3 == 0
}
~~~

Wait, whaaaat? Remember,the last thing in a function is a return in Rust,
but there's one wrinkle: note there's no semicolon here. If you had one,
you'd get:

    $ rustc --test fizzbuzz.rs
    fizzbuzz.rs:2:15: 2:16 note: consider removing this semicolon:
    fizzbuzz.rs:2     num % 3 == 0;
                                ^
    fizzbuzz.rs:1:1: 3:2 error: not all control paths return a value
    fizzbuzz.rs:1 fn div_by_three(num: int) -> bool {
    fizzbuzz.rs:2     num % 3 == 0;
    fizzbuzz.rs:3 }
    error: aborting due to previous error


Basically, ending an expression in Rust with a semicolon ignores the
value of that expression. Another way to think about it is that the
semicolon turns the expression into a statement, and statements don't
have values. This is kinda weird. It becomes natural after some use, though.
And Rust is even smart enough to tell us that it's probably a problem!

Okay, now try to TDD out the `div_by_five` and `div_by_fifteen` methods. They
should work the same way, but this will let you get practice actually
writing it out. Once you see this, you're ready to advance:

    $ rustc --test fizzbuzz.rs && ./fizzbuzz

    running 6 tests
    test test_div_by_fifteen ... ok
    test test_div_by_five_with_five ... ok
    test test_div_by_five ... ok
    test test_div_by_fifteen_with_fifteen ... ok
    test test_div_by_three ... ok
    test test_div_by_three_with_three ... ok

    test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured


Okay! Let's talk about the main program now. We've got the tools to
build FizzBuzz, let's make it work. First thing we need to do is print
out all the numbers from one to 100. It's easy!

~~~ {.rust}
    fn main() {
        for num in range(1i, 100) {
            println!("num");
        }
    }
~~~

Step one: print **something** 100 times. If you run this via
`rustc fizzbuzz.rs && ./fizzbuzz` you should see `num` printed
100 times. Note that our tests didn't actually run. Not only are they not run,
they're actually not even in the executable:

Compile with the test flag:

~~~
$ rustc --test fizzbuzz.rs
~~~

On Linux:

~~~
$ nm -C fizzbuzz | grep test
~~~

On OS X:

~~~
$ nm fizzbuzz | c++filt -p -i | grep test
~~~

Expected output:

~~~
0000000000403cd0 t test_div_by_five_with_five::_79fbef3fc431adf6::_00
0000000000403ac0 t test_div_by_three_with_three::_79fbef3fc431adf6::_00
0000000000403c10 t test_div_by_five_with_not_five::_79fbef3fc431adf6::_00
0000000000403ee0 t test_div_by_fifteen_with_fifteen::_79fbef3fc431adf6::_00
0000000000403a00 t test_div_by_three_with_not_three::_79fbef3fc431adf6::_00
0000000000403e20 t test_div_by_fifteen_with_not_fifteen::_79fbef3fc431adf6::_00
                 U test::test_main_static::_e5d562a4bc8c4dd6::_06
000000000040fea0 T __test::main::_79fbef3fc431adf6::_00
0000000000614890 D __test::tests::_7c31a8a9617a6a::_00
~~~

Compile without the test flag:

~~~
$ rustc fizzbuzz.rs
~~~

On Linux:

~~~
$ nm -C fizzbuzz | grep test
~~~

On OS X:

~~~
$ nm fizzbuzz | c++filt -p -i | grep test
~~~

Expected output:

    $

Neat, huh? Rust is smart.

Anyway, `nm`: The `nm` program lists all the symbols in a binary executable or
library. The `-C` option is important on linux, it "de-mangles" the symbol names.
On OS X, `nm` provides no symbol de-mangling option, so the output must be piped to `c++filt`. Rust
uses the same mangling scheme as C++, so it's compatible with all the existing
tools. How it works isn't that important, though.  It's cool low-level stuff if
you're into that sort of thing.

Anywho, where were we? Oh, iteration:

~~~ {.rust}
fn main() {
    for num in range(1i, 100) {
        println!("{:d}", num);
    }
}
~~~

This uses string interpolation: the double curlies tell Rust where to place
`num` in the string.

Anyway, now we have 1 to 99. We need 1 to 100.

~~~ {.rust}
fn main() {
    for num in range(1i, 101) {
        println!("{:d}", num);
    }
}
~~~

Now we can put the two together:

~~~ {.rust}
fn main() {
    for num in range(1i, 101) {
        let mut answer = "";

        if div_by_fifteen(num){
            answer = "FizzBuzz";
        }
        else if div_by_three(num) {
            answer = "Fizz";
        }
        else if div_by_five(num) {
            answer = "Buzz";
        }
        else {
            answer = "";
        };

        println!("{:s}", answer);
    }
}
~~~

Uhhhh `let mut`? `let` is the way that we make a local variable. `mut`
means we plan to mutate that variable: yes, variables are immutable by
default.

Also, `:s` is the format string for a... string.

We can shorten this up a bit with this syntax:

~~~ {.rust}
fn main() {
    for num in range(1i, 101) {
        let mut answer =
            if div_by_fifteen(num){
                "FizzBuzz"
            }
            else if div_by_three(num) {
                "Fizz"
            }
            else if div_by_five(num) {
                "Buzz"
            }
            else {
                ""
            };

        println!("{:s}", answer);
    }
}
~~~

We've made the `if` assign the value to answer. Note that we had to
remove the semicolons again; that lets the expression give its value to
`answer.` Note that this _also_ makes answer immutable, so we can
remove the `mut`:

~~~ {.rust}
fn main() {
    for num in range(1i, 101) {
        let answer =
            if div_by_fifteen(num){
                "FizzBuzz"
            }
            else if div_by_three(num) {
                "Fizz"
            }
            else if div_by_five(num) {
                "Buzz"
            }
            else {
                ""
            };

        println!("{:s}", answer);
    }
}
~~~

Not too shabby! I love eliminating mutable state.

Of course, this version gives us lots of empty lines, so what we
actually want is:

~~~ {.rust}
fn main() {
    for num in range(1i, 101) {
        let answer =
            if div_by_fifteen(num){
                "FizzBuzz".to_string()
            }
            else if div_by_three(num) {
                "Fizz".to_string()
            }
            else if div_by_five(num) {
                "Buzz".to_string()
            }
            else {
                num.to_string()
            };

        println!("{}", answer);
    }
}
~~~

Why the "`to_string()`"s? There are two types of Strings in Rust: `String`,
which is a heap allocated string with dynamic length, and `&str`, which
is a borrowed, immutable view into a string. The literal is of type `&str`,
but we want a `String`. `to_string()` turns a `&str` into a `String`.

Before, we could get away with a `&str`, because they all had the same
type. But since we've added an arm with an `int`, we need to make them all
the same type, and there's no way to convert an `int` into a `&str`.

Because the `if` returns a value, we could also do something like this:

~~~ {.rust}
fn main() {
    for num in range(1i, 101) {
        println!("{:s}", 
            if div_by_fifteen(num) { "FizzBuzz".to_string() }
            else if div_by_three(num) { "Fizz".to_string() }
            else if div_by_five(num) { "Buzz".to_string() }
            else { num.to_string() }
        );
    }
}
~~~

It's more compact, and removes the intermediate variable all together.

We can do one other thing too: this whole `if/fail!` thing so common in
tests seems too complex. Why do we have to write if over and over and
over again? Meet `assert!`:

~~~ {.rust}
#[test]
fn test_div_by_fifteen_with_fifteen() {
    assert!(div_by_fifteen(15))
}
~~~

This will fail if it gets false, and pass if it gets true. Simple! You
can also give it a message to be printed when the assertion fails,
mostly useful when you are using `assert!` to test for preconditions and
such:

~~~ {.rust}
fn main() {
    assert!(1 == 0, "1 does not equal 0!");
}
~~~

Try running it.

Anyway, awesome! We've conquered FizzBuzz.
