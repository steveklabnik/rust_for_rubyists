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
    let m = Monster { health:10, attack:20 };

    io::println(int::str(m.health));
    io::println(int::str(m.attack));
  }

This gives::

  $ make
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz
  10
  20

Seems simple enough!

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
        io::println(fmt!("The monster attacks for %d damage.", self.attack));
      }
  }

  fn main() {
    let m = Monster { health:10, attack:20 };

    m.attack();
  }

This gives::

  $ make
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz
  The monster attacks for 20 damage.

Simple! What about this part?::

  fmt!("The monster attacks for %d damage.", self.attack)

The bang on the end of fmt indicates that it's a 'synatax extension.' I don't
know what that means yet. It's basically just another function as far as I can
tell. We'll look into it later. But it formats just like ``sprintf`` does: put
some sort of ``%d`` into the string and pass it a digit, and you get a number
in the middle. Easy enough.

Methods will want to take a borrowed pointer, obviously. We don't care what
the ownership semantics are. That's the ``&self``, if you forgot.

You can define static methods as well::

  struct Monster {
      health: int,
      attack: int
  }

  impl Monster {
      fn attack(&self) {
        io::println(fmt!("The monster attacks for %d damage.", self.attack));
      }

      static fn count() {
        io::println("There are a bunch of monsters out tonight.");
      }
  }

  fn main() {
    let m = Monster { health:10, attack:20 };

    m.attack();
    Monster::count();
  }

Right now, you need that ``static`` keyword, but eventually, the language will
just figure it out depending if you define ``self``. Neato!

Constructors are a good reason to use static methods::

  struct Monster {
      health: int,
      attack: int
  }

  impl Monster {
      fn attack(&self) {
        io::println(fmt!("The monster attacks for %d damage.", self.attack));
      }

      static fn count() {
        io::println("There are a bunch of monsters out tonight.");
      }

      static fn new(health: int, attack: int) -> Monster {
        Monster { health:health, attack:attack }
      }
  }

  fn main() {
    Monster::new(20, 40).attack();
  }

This gives::

  $ make
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz
  The monster attacks for 40 damage.

as you'd expect.

Enums
-----

What if we want to define a few different types of things? In other langauges,
we'd use inheritance. In Rust, it seems like Enums are a better idea. Here's
an enum::

  enum Monster {
    ScubaArgentine(int, int, int, int),
    IndustrialRaverMonkey(int, int, int, int)
  }


  impl Monster {
    fn attack(&self) {

      match *self {
        ScubaArgentine(l, s, c, w) => io::println(fmt!("The monster attacks for %d damage.", w)),
        IndustrialRaverMonkey(l, s, c, w) => io::println(fmt!("The monster attacks for %d damage.", w))
      }
    }
  }

  fn main() {
    let irm = IndustrialRaverMonkey(46, 35, 91, 2);
    irm.attack();
  }

Okay, few new things here: We can see that there's some duplication here.
Obviously this isn't the best way to do it, but I wanted to try this out before
we got to the better implemenation. We make an ``Enum`` that defines two
different things, and then we use this ``match`` expression to decompose
things.

If you haven't used pattern matching in another language, you're missing out.
It's awesome. Here's a simpler match expression::

  fn message(i: int) {
    match i {
      1 => io::println("ONE!"),
      2 => io::println("Two is a prime."),
      3 => io::println("THREE!"),
      _ => io::println("no idea what that is, boss")
    }
  }

  fn main() {
    message(1);
    message(2);
    message(3);
  }

Does that make sense? It's sorta like a ``case`` statement, but it's more
powerful. If we leave off the ``_`` case, Rust will complain::

  $ make
  rustc fizzbuzz.rs
  fizzbuzz.rs:2:2: 6:3 error: non-exhaustive patterns
  fizzbuzz.rs:2   match i {
  fizzbuzz.rs:3     1 => io::println("ONE!"),
  fizzbuzz.rs:4     2 => io::println("Two is a prime."),
  fizzbuzz.rs:5     3 => io::println("THREE!")
  fizzbuzz.rs:6   }
  error: aborting due to previous error
  make: *** [build] Error 101

Neat. The cool thing is that when pattern matching on a struct, the ``match``
can destruct it::

  match p {
    Point(x, y) => io::println(fmt!("X: %d, Y: %d", x, y))
  }
  
We name the two fields of a ``Point`` ``x`` and ``y``, and those names are
valid within the match expression.

Let's build monsters!
---------------------

Before we build some monsters, let's look at the right way to implement them.
We can do this with Traits, but that's the next chapter.

.. _DwemthysArray: http://mislav.uniqpath.com/poignant-guide/dwemthy/

