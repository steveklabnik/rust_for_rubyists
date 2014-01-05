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
    fn test_is_three() {
        if is_three(1) {
            fail!("One is not three");
        }
    }
~~~

And run it:

    $ rust test fizzbuzz.rs
    fizzbuzz.rs:3:7: 3:15 error: unresolved name `is_three`.
    fizzbuzz.rs:3     if is_three(1) {
                         ^~~~~~~~
    error: aborting due to previous error

This makes sense: We haven't defined any functions yet. Let's define
one:

~~~ {.rust}
    fn is_three(num: int) -> bool {
        return true;
    }

    #[test]
    fn test_is_three() {
        if is_three(1) {
            fail!("One is not three");
        }
    }
~~~

Okay. Here's some new syntax. The `num: int` says that we take one
argument, `num`, and that it's of an integer type. The `-> bool` says
that we return a boolean, and the `return true;`, well, returns true.

You'll also note we have an `if` statement. It's pretty close to what
you'd expect, but we have curly braces rather than our friends `do/end`.

Now that we've got that cleared up, let's run our tests:

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

Rust is kind enough to give us a warning: we never used the `num`
argument. We then get our failure, "One is not three", because we
returned true. Now that we have a failing test, let's make it pass:

~~~ {.rust}
    fn is_three(num: int) -> bool {
      return false;
    }

    #[test]
    fn test_is_three() {
        if is_three(1) {
            fail!("One is not three");
        }
    }
~~~

TDD means do the simplest thing! And run it:

    $ rust test fizzbuzz.rs
    fizzbuzz.rs:1:12: 1:16 warning: unused variable: `num`
    fizzbuzz.rs:1 fn is_three(num: int) -> bool {
                              ^~~~

    running 1 test
    test test_is_three ... ok

    result: ok. 1 passed; 0 failed; 0 ignored

Awesome! We pass! We still have that warning, though... let's write
another test, and see what happens:

~~~ {.rust}
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
        fail!("Three should be three");
      }
    }
~~~

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

Great! It showed that our first test passed, and that our second one
failed. Let's make both tests pass:

~~~ {.rust}
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
~~~

    $ rust test fizzbuzz.rs

    running 2 tests
    test test_is_three_with_not_three ... ok
    test test_is_three_with_three ... ok

    result: ok. 2 passed; 0 failed; 0 ignored

Awesome! This shows off how elses work, as well. It's probably what you
expected. Go ahead and try to refactor this into a one-liner.

Done? How'd you do? Here's mine:

~~~ {.rust}
    fn is_three(num: int) -> bool {
        num % 3 == 0
    }
~~~

Wait, whaaaat? Yep, the last thing in a function is a return in Rust,
but there's one wrinkle: note there's no semicolon here. If you had one,
you'd get:

    $ rust test fizzbuzz.rs

    fizzbuzz.rs:1:0: 3:1 error: not all control paths return a value
    fizzbuzz.rs:1 fn is_three(num: int) -> bool {
    fizzbuzz.rs:2     num % 3 == 0;
    fizzbuzz.rs:3 }
    error: aborting due to previous error

Basically, ending an expression in Rust with a semicolon ignores the
value of that expression. Another way to think about it is that the
semicolon turns the expression into a statement, and statements don't
have values. This is kinda weird. I don't know how I feel about it. But
it is something you should know about.

Okay, now try to TDD out the `is_five` and `is_fifteen` methods. They
should work the same way, but this will let you get practice actually
writing it out. Once you see this, you're ready to advance:

    $ rust test fizzbuzz.rs

    running 6 tests
    test test_is_five_with_not_five ... ok
    test test_is_fifteen_with_fifteen ... ok
    test test_is_three_with_not_three ... ok
    test test_is_five_with_five ... ok
    test test_is_three_with_three ... ok
    test test_is_fifteen_with_not_fifteen ... ok

    result: ok. 6 passed; 0 failed; 0 ignored

Okay! Let's talk about the main program now. We've got the tools to
build FizzBuzz, let's make it work. First thing we need to do is print
out all the numbers from one to 100. It's easy!

~~~ {.rust}
    fn main() {
        for num in range(0,100) {
            println("num");
        }
    }
~~~

Step one: print **something** 100 times. If you run this via
`rustc fizzbuzz.rs && ./fizzbuzz` you should see `num` printed
100 times. Note that our tests didn't actually run. Not only are they not run,
they're actually not even in the executable:

On Linux:

~~~
$ nm -C fizzbuzz | grep test
~~~

On OS X:

~~~
$ nm fizzbuzz | c++filt -p -i | grep test
~~~
   
~~~
0000000000403cd0 t test_is_five_with_five::_79fbef3fc431adf6::_00
0000000000403ac0 t test_is_three_with_three::_79fbef3fc431adf6::_00
0000000000403c10 t test_is_five_with_not_five::_79fbef3fc431adf6::_00
0000000000403ee0 t test_is_fifteen_with_fifteen::_79fbef3fc431adf6::_00
0000000000403a00 t test_is_three_with_not_three::_79fbef3fc431adf6::_00
0000000000403e20 t test_is_fifteen_with_not_fifteen::_79fbef3fc431adf6::_00
                 U test::test_main_static::_e5d562a4bc8c4dd6::_06
000000000040fea0 T __test::main::_79fbef3fc431adf6::_00
0000000000614890 D __test::tests::_7c31a8a9617a6a::_00
~~~

~~~
$ rustc fizzbuzz.rs
~~~

On Linux:

~~~
$ nm -C fizzbuzz~ | grep test
~~~

On OS X:

~~~
$ nm fizzbuzz~ | c++filt -p -i | grep test
~~~

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
        for num in range(0, 100) {
            println(num)
        }
    }
~~~


Note we've removed the quotes to print the number itself rather than the string
'num'.  If we run this, we get another error message:

    $ rustc fizzbuzz.rs && ./fizzbuzz
    fizzbuzz.rs:46:16: 46:19 error: error: mismatched types: expected `&str`
       but found `int` (expected &str but found int)

    fizzbuzz.rs:46         println(num);
                                   ^~~
    error: aborting due to previous error

Mismatched types: expected &str but found int. It wants a string, but we gave
it a number. Whoops! Now, there's two ways to fix this. The first is to use the
`to_str` function:

~~~ {.rust}
    fn main() {
        for num in range(1, 100) {
            println(num.to_str())
        }
    }
~~~

Awesome. Let's run it:

    $ rust run fizzbuzz.rs
    1
    2
    3
    ...
    99

Bam! Whew. We had to fight with the compiler a bit, and the errors
weren't great, but that wasn't too bad. The other way to do it is to use
the `format!` function. At least, it looks like a function to me. Here it
is:

~~~ {.rust}
    fn main() {
      for num in range(1, 4) {
        println(format!("{:d}", num));
      }
    }
~~~

`format!` is similar to `str % arg`, or the `format` and `sprintf`
functions in `Kernel`: it takes a format string, some arguments, and
makes a string out of them. A cool feature of rust that sets it apart
from C or C++, which also have this, is that the format strings are
type-checked at compile time. No more broken format strings!

Because this combination is common, you can use `println!` as a combination of
`println` and `format!`:

~~~ {.rust}
    fn main() {
      for num in range(1, 100) {
        println!("{:d}", num);
      }
    }
~~~

Anyway, now we have 1 to 99. We need 1 to 100.

~~~ {.rust}
    fn main() {
        for num in range(1, 101) {
            println!("{:d}", num);
        }
    }
~~~

Now we can put the two together:

~~~ {.rust}
    fn main() {
        for num in range(1, 101) {
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
~~~

Uhhhh `let mut`? `let` is the way that we make a local variable. `mut`
means we plan to mutate that variable: yes, variables are immutable by
default. When I first wrote this, I wrote this:

~~~ {.rust}
    let mut answer = "";
~~~

We can shorten this up a bit with this syntax:

~~~ {.rust}
    fn main() {
        for num in range(1, 101) {
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
~~~

We've made the `if` assign the value to answer. Note that we had to
remove the semicolons again; that lets the expression give its value to
`answer.` Note that this \_also\_ makes answer immutable, so we can
remove the `mut`:

~~~ {.rust}
    fn main() {
        for num in range(1, 101) {
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
~~~

Not too shabby! I love eliminating mutable state.

Of course, this version gives us lots of empty lines, so what we
actually want is:

~~~ {.rust}
    fn main() {
        for num in range(1, 101) {
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
~~~

What's up with the tildes? They modify the declaration somehow. I added it
because running without it gives an error message that implies you need it:
give it a shot. Because our variables are typed, we have to convert the number
in the `else` case to a string. In Ruby we'd just let it be a `Fixnum` if it
was a number. Oh well. We'll talk more about them later.

Because the `if` returns a value, we could also do something like this:

~~~ {.rust}
    fn main() {
        for num in range(1, 101) {
            println(
              if is_fifteen(num) { ~"FizzBuzz" }
              else if is_three(num) { ~"Fizz" }
              else if is_five(num) { ~"Buzz" }
              else { num.to_str() }
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
    fn test_is_fifteen_with_fifteen() {
      assert!(is_fifteen(15))
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

Anyway, awesome! We've conquered FizzBuzz. `is_fifteen` isn't the best
named method, but we're just learning. ;) You can check the code samples
for the full, final code.
