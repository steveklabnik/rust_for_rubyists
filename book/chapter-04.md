Testing
=======

Rubyists love testing, so before we go any farther, let's talk about
testing. In Rust, there is a unit testing framework built in, and it's
pretty simple. Let's write some very simple code and tests to exercise
it.

In Rust, you annotate test methods like such:

~~~ {.rust}
#[test]
fn this_tests_code() {
    // SOMETHING HERE
}
~~~

You'll note that tests take no arguments and return nothing. If the
function runs, the test passes, and if it errors in some way, the test
fails. Let's give it a shot: Open up `testing.rs` and put this in it:

~~~ {.rust}
#[test]
fn this_tests_code() {
    println!("");
}
~~~

Then, use `rustc` with a special flag:

    $ rustc --test testing.rs

This tells `rustc` to compile your tests, and replaces the `main` function
with a test runner. Try it out:

    $ ./testing

You should get some output that looks like this:

    running 1 test

    test this_tests_code ... ok

    result: ok. 1 passed; 0 failed; 0 ignored

Bam! Now let's make it fail:

~~~ {.rust}
#[test]
fn this_tests_code() {
    panic!("Fail!");
}
~~~

Recompile, and the output should be:

    running 1 test
    test this_tests_code ... FAILED

    failures:

    ---- this_tests_code stdout ----
      task 'this_tests_code' failed at 'Fail!', testing.rs:5
      

    failures:
        this_tests_code

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

    task '<main>' failed at 'Some tests failed', /some/path/to/something


Super simple. That's all you need to know to get started. Next up: FizzBuzz.
