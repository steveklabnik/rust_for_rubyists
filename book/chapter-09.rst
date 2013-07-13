{% import 'macros/ork.jinja' as ork with context %}

Structs and Methods
===================

I'd like to talk about structs and methods, so let's build a fun little
project: DwemthysArray_. One of _why's sillier examples, we make an array of
monsters, and then fight them. We won't be building the Array _exactly_, but
something like it.

Structs
-------

Structs are ways of packaging up multiple values into one::

  struct Monster {
      health: int,
      attack: int
  }

  fn main() {
    let m = Monster { health: 10, attack: 20 };

    println(m.health.to_str());
    println(m.attack.to_str());
  }

This gives::

  $ rust run dwemthysarray.rs
  10
  20

Seems simple enough! Note that you can give a struct to ``fmt!``, using the
``%?`` format specifier. Example::

  $ rust run dwemthysarray.rs
  {health: 10, attack: 20}

Fancy!

Methods
-------

Methods are basically functions that take a first argument named ``self``.
Python people who are reading will be high fiving each other in droves. Let's
add a method for our ``Monster`` s::

  struct Monster {
      health: int,
      attack: int
  }

  impl Monster {
      fn attack(&self) {
          println(fmt!("The monster attacks for %d damage.", self.attack));
      }
  }

  fn main() {
      let m = Monster { health: 10, attack: 20 };

      m.attack();
  }

This gives::

  $ rust run dwemthysarray.rs
  The monster attacks for 20 damage.

Methods will want to take a borrowed pointer, obviously. We don't care what
the ownership semantics are. That's the ``&self``, if you forgot.

You can define associated functions (class methods, in Ruby, static methods,
in Java)  as well::

  struct Monster {
      health: int,
      attack: int
  }

  impl Monster {
      fn attack(&self) {
          println(fmt!("The monster attacks for %d damage.", self.attack));
      }

      fn count() {
          println("There are a bunch of monsters out tonight.");
      }
  }

  fn main() {
      let m = Monster { health: 10, attack: 20 };

      m.attack();
      Monster::count();
  }

Constructors are a good reason to use associated functions::

  struct Monster {
      health: int,
      attack: int
  }

  impl Monster {
      fn new(health: int, attack: int) -> Monster {
          Monster { health:health, attack:attack }
      }

      fn attack(&self) {
          println(fmt!("The monster attacks for %d damage.", self.attack));
      }

      fn count() {
          println("There are a bunch of monsters out tonight.");
      }

  }

  fn main() {
      Monster::new(20, 40).attack();
  }

This gives::

  $ rust run dwemthysarray.rs
  The monster attacks for 40 damage.

as you'd expect.

Enums
-----

What if we want to define a few different types of things? In other languages,
we'd use inheritance. In Rust, it seems like Enums are a better idea. Here's
an enum::

  enum Monster {
      ScubaArgentine(int, int, int, int),
      IndustrialRaverMonkey(int, int, int, int)
  }


  impl Monster {
      fn attack(&self) {
        match *self {
            ScubaArgentine(l, s, c, w) => println(fmt!("The monster attacks for %d damage.", w)),
            IndustrialRaverMonkey(l, s, c, w) => println(fmt!("The monster attacks for %d damage.", w))
        }
      }
  }

  fn main() {
      let irm = IndustrialRaverMonkey(46, 35, 91, 2);
      irm.attack();
  }


Okay, few new things here: We can see that there's some duplication here.
Obviously this isn't the best way to do it, but I wanted to try this out before
we got to the better implementation. We make an ``Enum`` that defines two
different things, and then we use this ``match`` expression to "destructure"
them and get at their... well, members, sorta.

If you haven't used pattern matching in another language, you're missing out.
It's awesome. Here's a simpler match expression::

  fn message(i: int) {
    match i {
        1 => println("ONE!"),
        2 => println("Two is a prime."),
        3 => println("THREE!"),
        _ => println("no idea what that is, boss")
      }
  }

  fn main() {
      message(1);
      message(2);
      message(3);
  }

Does that make sense? It's sorta like a ``case`` statement, but it's more
powerful. If we leave off the ``_`` case, Rust will complain::

  $ rust run match.rs
  match.rs:2:4: 6:5 error: non-exhaustive patterns
  match.rs:2     match i {
  match.rs:3         1 => println("ONE!"),
  match.rs:4         2 => println("Two is a prime."),
  match.rs:5         3 => println("THREE!"),
  match.rs:6     }
  error: aborting due to previous error

Neat. The cool thing is that when pattern matching on a struct, the ``match``
can destructure it::

  match p {
      Point(x, y) => println(fmt!("X: %d, Y: %d", x, y))
  }

We name the two fields of a ``Point`` ``x`` and ``y``, and those names are
valid within the match expression. Match is a lot more powerful (they can
express ranges, options, and even variable binding), but this is its common
use.

Let's build monsters!
---------------------

Before we build some monsters, let's look at the Right Way to implement them.
We can do this with Traits, but that's the next chapter.

.. _DwemthysArray: http://mislav.uniqpath.com/poignant-guide/dwemthy/
