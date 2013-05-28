{% import 'macros/ork.jinja' as ork with context %}

Standard Input
==============

If we want to make this little text-based game, we need to figure out how to
get text off of standard in. So let's do another little programming project
I enjoy when learning a new language: the numbers guessing game.

Guessing Game
-------------

The guessing game is really simple: You enter in a number between 1 and 100.
The computer tells you if you're too low, too high, or just right. You get
five tries, after which the computer tells you the answer if you haven't
gotten it yet.

I pick this example because it's fun, not too hard, and lets us do text-based
I/O with a teeny bit of logic. Let's go!

Using ``stdin()``
-----------------

Turns out getting text input is pretty simple. Just try this::

  use core::io::{Reader, ReaderUtil};
  fn main() {
      println("INPUT:");
      let in = io::stdin().read_line();

      println("YOU TYPED:");
      println(in);
  }

Give that a run. It should prompt you to type something in, and then echo out
what you typed. Simple enough!

I want to talk about that import, but first, let's go over this ``stdin()``
business. Basically. ``io::stdin()`` will give you a reference to standard in.
It implements an interface called ``Reader``, which gives you access to the
``read_line()`` method. This reads stuff up to a ``\n`` from whatever it's
implemented on. So we grab that line, save it in a variable, and then print
it out again. Super simple.

The only thing that's annoying from Ruby is that you must type the ``()`` s
when you use methods. So this won't work::

  let in = io::stdin.read_line;

  $ make          
  rustc fizzbuzz.rs
  fizzbuzz.rs:123:11: 123:30 error: attempted to take value of method (try writing an anonymous function)
  fizzbuzz.rs:123   let in = io::stdin.read_line;
                             ^~~~~~~~~~~~~~~~~~~

Oh well. It's not the end of the world.

Anyway, what's up with this ``use`` shenanigans?

How to use ``use``
------------------

Let's talk about modules. One of the big things that sorta sucks about C (and
Ruby) is that 'modules' are basically based on files. You include the file,
and that's about it. There's no way to really qualify "I want this bit of
code," you say "I want this file that happens to have this code in it."

Rust basically pretends that it has these two lines at the beginning of every
program::

  extern mod core;
  use core::*;

Two things here. The first line is this ``extern mod`` business. I wanted to
clarify my understanding, so I jumped into the `ever helpful Rust IRC`_ and
asked::

  pcwalton:	basically "extern mod" is where you put the stuff you'd put on the compiler link line in C++

Right. So we're saying 'please link against this library.' Rust uses a load
path to find where those libraries are, which you can modify with the ``-L``
command-line flag. For instance::

   $ rustc foo.rs -o foo -L ./lib

Would compile ``foo.rs`` into ``foo`` while also looking for extra libraries
in the ``lib`` directory. These libraries are called 'crates' in Rust, and you
can make one of your own with the ``--lib`` flag to ``rustc``::

  $ rustc bar.rs -o bar --lib

This would make a shared library crate named ``bar``. Technically, any time
you compile something, it makes a crate: the ``--lib`` flag just says that
we're making a shared library explicitly, so Rust won't look for a ``main()``.
When you invoke ``rustc`` normally, you're also building a crate, it's just not
shared.

Okay, so, once you've imported a crate, what do you get? Well, it will put a
module with the same name as the crate into the current scope. But crates can
also have other modules, which only get imported when you qualify them.

Modules?

Every Rust file can contain one top-level module, and modules can contain other
modules. Modules look like this::

  mod foo {
      pub fun bar() { "bar" }
      pub fun baz() { "baz" }
      pub fun qux() { "qux" }
  }

You just shove a ``mod`` around everything that goes in the module. To bring
``bar`` into scope, you::

  use foo::bar;

To bring them all into scope, you::

  use foo::*;

To bring ``bar`` and ``baz`` into scope, but not ``qux`` you do either one of
these::

  use foo::bar;
  use foo::baz;

  use foo::{bar,baz};

Pretty simple. So now we can see why the code acts like it has these two lines
at the top::

  extern mod core;
  use core::*;

We want to link against the core library, and then import all the default stuff
into scope. This is why we need::

  use io::{Reader, ReaderUtil};

We're bringing these two interfaces into scope. Not everything in ``io`` needs
them, so they're not imported by default. And we use the ``{,}`` syntax in
order to do it on one line.

Casting to integer
------------------

So, I was trying to cast a string to an integer to get this program going. So
I wrote this::

  use core::io::{Reader, ReaderUtil};

  fn main() {
      let in = io::stdin().read_line();
      println("INPUT:");
      println(int::str(int::from_str(in)));
  }

I was gonna convert the string to an int, then back to a string to print it out
to the screen.

This threw an error::

  $ make
  rustc fizzbuzz.rs
  fizzbuzz.rs:125:23: 125:41 error: mismatched types: expected `int` but found `core::option::Option<int>` (expected int but found enum core::option::Option)
  fizzbuzz.rs:125   println(int::str(int::from_str(in)));
                                         ^~~~~~~~~~~~~~~~~~
  error: aborting due to previous error
  make: *** [build] Error 101

Wait, huh? Here's the thing: Rust **knows** that we might have a string that
doesn't make any sense as an integer. For example: ``"foo"``. So it doesn't
actually return a string, it returns an ``Option``. We can then use pattern
matching to handle both cases. Observe::

  use core::io::{Reader, ReaderUtil};

  fn main() {
      let in = io::stdin().read_line();

      match int::from_str(in) {
          Some(number_string) => println(int::to_str(number_string)),
          None                => println("Hey, put in a number.")
      }
  }

Remember ``match``? It's really good for matching against some kind of type and
breaking it up. Here we match against our ``Option`` type. ``Option`` looks
like this::

  enum Option<T> {
      Some(T),
      None
  }

``Option`` is called ``Maybe`` in some other languages, but basically, you
can think of it as a type that handles what we'd use ``nil`` for in Ruby.
We may have ``Some(int)``, but we also may have ``None``. Computations that
may fail in some way should return ``None`` if it fails. Simple. We can't ever
ignore a possible failure: the type system makes us handle it.

Looping forever
---------------

Looping forever is possible with ``while true``, but like in Ruby, that's
kinda silly. Rust gives us ``loop`` to loop forever::

  loop {
      println("HELLO")
  }

Obviously you don't want to actually run that. You can use ``break`` to break
out of the loop::

  let mut i = 0;
  loop {
      i += 1;
      if i == 5 { break; }
      println("hi");
  }

This will print ``"hi"`` five times. You're going to want to do this, because
if someone mis-types a number, we don't want to count it against them: we
should just ask them to put in another number.

Random Number Generation
------------------------

Random number generation isn't too bad::

  use core::rand::RngUtil;

  fn main() {
      let rng = rand::Rng();
      println(int::to_str(rng.gen_int()));
  }

This will print out a different number each time you run it. But you'll get
biiiiiiig numbers. If we want 1-100, of course we have to do this::

  use core::rand::RngUtil;

  fn main() {
      let rng = rand::Rng();
      let num = rng.gen_int() % 100 + 1;
      println(int::to_str(num));
  }

One issue with this: We can get negatives too. ``abs`` to the rescue!!!::

  use core::rand::RngUtil;
  use core::int::abs;

  fn main() {
      let rng = rand::Rng();
      let num = abs(rng.gen_int() % 100) + 1;
      println(int::to_str(num));
  }

This will get us a random number between 1 and 100.

Okay! You should have all the tools you need to implement the guessing game.
Have it it. I'm starting... now.

My version
----------

Okay! That took me... about half an hour. Maybe 45 minutes. I decided to use
some pointer stuff...  Check it out::

  use core::io::{Reader, ReaderUtil};
  use core::rand::RngUtil;
  use core::int::abs;
  
  fn generate_secret_number() -> int {
      abs(rand::Rng().gen_int() % 100) + 1
  }

  fn process_guess(secret:int, guess: int, guesses: &mut int) {
      println(fmt!("You guessed: %d", guess));

      if guess > secret {
          println("Your guess was too high!");
      }
      else if guess < secret {
          println("Your guess was too low!");
      }
      else if guess == secret {
          println("You got it!");
          *guesses = 4; // this will end up ending the game.
      }

      *guesses += 1;
  }

  fn main() {
      let secret = generate_secret_number();
      
      let guesses = @mut 1;

      println("--- N U M B E R - G A M E ---");
      println("");
      println("Guess a number from 1-100 (you get five tries):");

      loop {
          println(fmt!("Guess #%d", *guesses));

          let in = io::stdin().read_line();

          match int::from_str(in) {
              Some(number) => process_guess(secret, number, guesses),
              None         => println("Hey, put in a number.")
          }
          if *guesses == 5 { break; }
      }

      println("Done!");
  }

That's it! I thought this was a little awkward, though: ``process_guess``
shouldn't really be worrying about mutating ``guesses``, which leads to all
kinds of awkward pointer stuff, as you can see. After asking on IRC, 'strcat'
gave me this version::

  use core::io::{Reader, ReaderUtil};
  use core::rand::RngUtil;
  use core::int::abs;

  fn generate_secret_number() -> int {
      abs(rand::Rng().gen_int() % 100) + 1
  }

  fn process_guess(secret:int, guess: int) -> bool {
      println(fmt!("You guessed: %d", guess));

      if guess > secret {
          println("Your guess was too high!");
          false
      } else if guess < secret {
          println("Your guess was too low!");
          false
      } else {
          println("You got it!");
          true
      }
  }

  fn main() {
      let secret = generate_secret_number();

      println("--- N U M B E R - G A M E ---");
      println("");
      println("Guess a number from 1-100 (you get five tries):");

      for int::range(0, 5) |round| {
          println(fmt!("Guess #%d", round));

          let in = io::stdin().read_line();

          match int::from_str(in) {
              Some(number) => {
                  if process_guess(secret, number) { break; }
              }
              None         => println("Hey, put in a number.")
          }
      }

      println("Done!");
  }

I like this better. We loop over a ``range`` that really shows we get 5 rounds,
we don't need to pass around silly pointers, and our dubiously-named
``process_guess`` now tells us if we need to quit the game.

Conclusion
----------

I'm pretty sure at this point we have basically everything I was able to do as
a child when programming stuff. You know enough of Rust now to be able to make
silly little games and scripts. This is obviously neat, but from this point
on, it's more about libraries, style, and solving things in an idiomatic way
than it is learning syntax. Of course, this was not a complete introduction to
the language, but this is the end of the 'beginner level' stuff. You should
have a basic idea of how to write many programs by this point. Pick a few
projects, try them out.

.. _`ever helpful Rust IRC`: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
