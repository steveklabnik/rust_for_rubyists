{% import 'macros/ork.jinja' as ork with context %}

Build toolchain
===============

Let's get a one-step build process going. We aren't used to the two-steps of
building and then running with Ruby, and while it's not a big deal to type
two things, we probably want to make it one step. Eventually, CI servers and
things will want a one-step build process, anyway.

Make
----

Yeah, you already know how to use Rake, but we're going to work with its
progenitor, ``make``. We don't want to assume that others have Ruby installed,
and you'll end up reading ``Makefiles`` that others write anyway, so it's time
to learn why Jim bothered with Rake in the first place.

Let's start off by running it::

  $ make
  make: *** No targets specified and no makefile found.  Stop.

Yep. Great error. Make can't find a Makefile. So let's make one::

  default:
    echo "hello world"

Yep. We define a target, in this case, 'default,' and then a colon, indent, and
then a bunch of shell commands we want to run. Try running make again now::

  $ make
  echo "hello world"
  hello world

Make will spit out all the commands it runs, so we see the echo and the result
of the echo. Let's tell ``make`` how to build our project. Since we're going to
build FizzBuzz in the next example, let's call it ``fizzbuzz.rs``::

  default:
    rustc fizzbuzz.rs

If we run ``make``, we get this output::

  $ make
  rustc fizzbuzz.rs
  error: error opening fizzbuzz.rs
  make: *** [default] Error 101

Let's make ``fizzbuzz.rs`` with the following contents::

  extern mod std;

  fn main() {
  }

And run make::

  $ make
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)

Awesome! It worked. This will build our project. Now we want to build it with
tests. We can do this::

  default:
    rustc fizzbuzz.rs

  test:
    rustc fizzbuzz.rs --test

And build it both ways::

  $ make
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)

  $ make test
  rustc fizzbuzz.rs --test
  warning: no debug symbols in executable (-arch x86_64)

Bam! So now we're compiling, but we're not running. Let's add the execution
into each of our tasks::

  default:
    rustc fizzbuzz.rs
    ./fizzbuzz

  test:
    rustc fizzbuzz.rs --test
    ./fizzbuzz

Now we can run it::

  $ make
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz

  $ make test
  rustc fizzbuzz.rs --test
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz

  running 0 tests

  result: ok. 0 passed; 0 failed; 0 ignored

This is cool, but what about duplication? We had to repeat that line. We can
fix this by making tasks depend on other tasks. Check this out::

  default: build run

  test: build-test run

  run:
    ./fizzbuzz

  build:
    rustc fizzbuzz.rs

  build-test:
    rustc fizzbuzz.rs --test

We define two low-level tasks, build and build-test. These do the compilation.
We define one other task, run, which handles the running. Then we make
default depend on build and then run, and test depend on build-test and then
run. Awesome. Let's run it::

  $ make
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz

Let's add a failing test to prove we've got it all. Edit
``fizzbuzz.rs`` and add this failing test at the end::

  #[test]
  fn this_tests_code() {
      fail ~"We just fail every time :-("
  }

Now, try our default ``make``::

  $ make
  rustc --test fizzbuzz.rs -o test-fizzbuzz
  ./test-fizzbuzz

  running 1 test
  rust: task failed at 'We just fail every time :-(', fizzbuzz.rs:9
  test this_tests_code ... FAILED

  failures:
      this_tests_code

  result: FAILED. 0 passed; 1 failed; 0 ignored

  running 0 tests

  result: ok. 0 passed; 0 failed; 0 ignored

Same thing. It worked. Cool.

You can do a lot more crazy stuff with Make, and we can make it only do partial
compilation, etc. I don't want to teach you everything about Make, this is a
book about Rust. So we'll just leave it like this for now. This recipe will
serve you well until you get to more than one file.

Next up: TDD-ing Fizzbuzz.
