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

Yep. Great error. We didn't tell ``make`` what we wanted ("No targets") and
there's no Makefile to provide a default target. We want ``fizzbuzz``.
Let's ask make for ``fizzbuzz``. Does make know what to do?::

  $ make fizzbuzz
  make: *** No rule to make target `fizzbuzz'.  Stop.

We've been mean to make. We haven't even given it a source file to
work with. Let's play nice for just a moment and give it something old
and familiar, a C program.::

  $ echo 'main() {}' > fizzbuzz.c
  $ make fizzbuzz
  cc     fizzbuzz.c   -o fizzbuzz

There we go! Make knows how to compile C programs. Notice that make
printed out the compile command that it ran. Is make going to know
about Rust? (Doesn't seem likely, does it?)

Let's make ``fizzbuzz.rs`` with the following contents::

  extern mod std;

  fn main() {
  }

And, don't forget to get the C program out of the way, both the source
and the compiled program::

  $ rm fizzbuzz.c fizzbuzz
  $ make
  make: *** No rule to make target `fizzbuzz'.  Stop.

Hmm, we've been here before. Make does not know about Rust programs.
We need to give it a rule. Create ``Makefile`` with two lines in it::

  fizzbuzz: fizzbuzz.rs
  rustc fizzbuzz.rs

Does it work?::

  $ make fizzbuzz
  rustc fizzbuzz.rs

Cool! We're, um, back where we started. We can compile a Rust program.
Boring. Let's run it, too. Add two more lines at the top of
``Makefile`` like this::

  run: fizzbuzz
  ./fizzbuzz

  fizzbuzz: fizzbuzz.rs
  rustc fizzbuzz.rs

We've added a rule ``run`` that depends on ``fizzbuzz``. Also, the
first rule in ``Makefile`` is the default.::

  $ make
  ./fizzbuzz

Just typing ``make`` ran our program. What happened to the compile
step? We told make that ``run`` depends on ``fizzbuzz``. Make noticed
that the compiled ``fizzbuzz`` is newer than the source file
``fizzbuzz.rs``. No new compilation needed! Let's check that make gets
this right. Edit fizzbuzz.rs to add a println statement::

  extern mod std;

  fn main() {
      io::println("Hello from Rust!");
  }

What does make do now?::

  $ make
  rustc fizzbuzz.rs
  ./fizzbuzz
  Hello from Rust!

Compile and run! Try it again?::

  $ make
  ./fizzbuzz
  Hello from Rust!

Ran it! And no compile needed.

Now, we want to setup for running tests as well. To keep things
simple, we'll just have a single source file ``fizzbuzz.rs`` for both
the program and the tests. We just want to compile it two different
ways. Add a compile rule at the end of ``Makefile`` for the testing
build, like this::

  test-fizzbuzz: fizzbuzz.rs
	rustc --test fizzbuzz.rs -o test-fizzbuzz

Does this work?::

  $ make test-fizzbuzz
  rustc --test fizzbuzz.rs -o test-fizzbuzz

Nice! Now add a "test" rule to run the tests::

  test: test-fizzbuzz
	./test-fizzbuzz

And give it a go::

  $ make test
  ./test-fizzbuzz

  running 0 tests

  result: ok. 0 passed; 0 failed; 0 ignored

For icing on the cake, define a default rule to "do it all". Here is
the whole ``Makefile``::

  all: test run

  run: fizzbuzz
	./fizzbuzz

  test: test-fizzbuzz
	./test-fizzbuzz

  fizzbuzz: fizzbuzz.rs
	rustc fizzbuzz.rs

  test-fizzbuzz: fizzbuzz.rs
	rustc --test fizzbuzz.rs -o test-fizzbuzz

The default is to run the tests. If the tests pass, run the program::

  $ make
  ./test-fizzbuzz

  running 0 tests

  result: ok. 0 passed; 0 failed; 0 ignored

  ./fizzbuzz
  Hello from Rust!

Let's add a failing test to prove we've got it all::

  $ make
  rustc --test fizzbuzz.rs -o test-fizzbuzz
  ./test-fizzbuzz

  running 1 test
  rust: task failed at 'We just fail every time :-(', fizzbuzz.rs:9
  test this_tests_code ... FAILED

  failures:
      this_tests_code

  result: FAILED. 0 passed; 1 failed; 0 ignored

  rust: task failed at 'Some tests failed', /tmp/rust-0.5/src/libstd/test.rs:62
  rust: domain main @0x15f0c00 root task failed
  rust: task failed at 'killed', /tmp/rust-0.5/src/libcore/task/mod.rs:570
  make: *** [test] Error 101

Yup. The failing test failed. And, make did not continue on to compile
and run the program. We still can ask make to run the program without
the tests::

  $ make run
  rustc fizzbuzz.rs
  ./fizzbuzz
  Hello from Rust!

You can do a lot more crazy stuff with Make, such as pattern rules. I
don't want to teach you everything about Make, this is a book about
Rust. So we'll just leave it like this for now. This recipe will serve
you well until you get to more than one file.

Next up: TDD-ing Fizzbuzz.
