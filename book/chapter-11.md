Standard Input
==============

If we want to make this little text-based game, we need to figure out
how to get text off of standard in. So let's do another little
programming project I enjoy when learning a new language: the numbers
guessing game.

Guessing Game
-------------

The guessing game is really simple: You enter in a number between 1 and
100. The computer tells you if you're too low, too high, or just right.
You get five tries, after which the computer tells you the answer if you
haven't gotten it yet.

I pick this example because it's fun, not too hard, and lets us do
text-based I/O with a teeny bit of logic. Let's go!

Using `stdin()`
---------------

Turns out getting text input is pretty simple. Just try this:

~~~ {.rust}
    use std::io;
    fn main() {
        println("INPUT:");
        let input = io::stdin().read_line();

        println("YOU TYPED:");
        println(input);
    }
~~~

Give that a run. It should prompt you to type something in, and then
echo out what you typed. Simple enough!

I want to talk about that import, but first, let's go over this
`stdin()` business. Basically. `io::stdin()` will give you a reference
to standard in. It implements an interface called `Reader`, which gives
you access to the `read_line()` method. This reads stuff up to a `\n`
from whatever it's implemented on. So we grab that line, save it in a
variable, and then print it out again. Super simple.

The only thing that's annoying from Ruby is that you must type the `()`
s when you use methods. So this won't work:

    let input = io::stdin().read_line;

    $ rust run fizzbuzz.rs
    /home/steveklabnik/tmp/foo.rs:4:16: 4:37 error: attempted to take value of method `read_line` on type `@std::io::Reader:'static` (try writing an anonymous function)

Oh well. It's not the end of the world.

Anyway, what's up with this `use` shenanigans?

How to use `use`
----------------

Let's talk about modules. One of the big things that sorta sucks about C
(and Ruby) is that 'modules' are basically based on files. You include
the file, and that's about it. There's no way to really qualify "I want
this bit of code," you say "I want this file that happens to have this
code in it."

Rust basically pretends that it has these two lines at the beginning of
every program:

~~~ {.rust}
    extern mod std;
    use std::prelude::*;
~~~

Two things here. The first line is this `extern mod` business. I wanted
to clarify my understanding, so I jumped into the [ever helpful Rust
IRC](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) and
asked:

    pcwalton: basically "extern mod" is where you put the stuff you'd put on the compiler link line in C++

Right. So we're saying 'please link against this library.' Rust uses a
load path to find where those libraries are, which you can modify with
the `-L` command-line flag. For instance:

    $ rustc -L ./lib -o foo foo.rs

Would compile `foo.rs` into `foo` while also looking for extra libraries
in the `lib` directory. These libraries are called 'crates' in Rust, and
you can make one of your own with the `--lib` flag to `rustc`:

    $ rustc --lib -o bar bar.rs

This would make a shared library crate named `bar`. Technically, any
time you compile something, it makes a crate: the `--lib` flag just says
that we're making a shared library explicitly, so Rust won't look for a
`main()`. When you invoke `rustc` normally, you're also building a
crate, it's just not shared.

Okay, so, once you've imported a crate, what do you get? Well, it will
put a module with the same name as the crate into the current scope. But
crates can also have other modules, which only get imported when you
qualify them.

Modules?

Every Rust file can contain one top-level module, and modules can
contain other modules. Modules look like this:

~~~ {.rust}
    mod foo {
        pub fn bar() { "bar" }
        pub fn baz() { "baz" }
        pub fn qux() { "qux" }
    }
~~~

You just shove a `mod` around everything that goes in the module. To
bring `bar` into scope, you:

~~~ {.rust}
    use foo::bar;
~~~

To bring them all into scope, you:

~~~ {.rust}
    use foo::*;
~~~

To bring `bar` and `baz` into scope, but not `qux` you do either one of
these:

~~~ {.rust}
    use foo::bar;
    use foo::baz;

    use foo::{bar,baz};
~~~

Pretty simple. So now we can see why the code acts like it has these two
lines at the top:

~~~ {.rust}
    extern mod std;
    use std::prelude::*;
~~~

We want to link against the core library, and then import all the
default io stuff into scope (that's what the prelude is). This is why we
need:

~~~ {.rust}
    use std::io;
~~~

Casting to integer
------------------

So, I was trying to cast a string to an integer to get this program
going. So I wrote this:

~~~ {.rust}
    use std::io;

    fn main() {
        let input = io::stdin().read_line();
        println("INPUT:");
        println(from_str::<int>(input).to_str());
    }
~~~

I was gonna convert the string to an int, then back to a string to print
it out to the screen.

This gave an odd result:

    $ make
    rustc fizzbuzz.rs
    5
    INPUT:
    Some(5)


Wait, huh? Here's the thing: Rust **knows** that we might have a string
that doesn't make any sense as an integer. For example: `"foo"`. So it
doesn't actually return a string, it returns an `Option`. We can then
use pattern matching to handle both cases. Observe:

~~~ {.rust}
    use std::io;

    fn main() {
        let input = io::stdin().read_line();

        match from_str::<int>(input) {
            Some(number_string) => println(number_string.to_str()),
            None                => println("Hey, put in a number.")
        }
    }
~~~

Remember `match`? It's really good for matching against some kind of
type and breaking it up. Here we match against our `Option` type.
`Option` looks like this:

~~~ {.rust}
    enum Option<T> {
        Some(T),
        None
    }
~~~

`Option` is called `Maybe` in some other languages, but basically, you
can think of it as a type that handles what we'd use `nil` for in Ruby.
We may have `Some(int)`, but we also may have `None`. Computations that
may fail in some way should return `None` if it fails. Simple. We can't
ever ignore a possible failure: the type system makes us handle it.

Looping forever
---------------

Looping forever is possible with `while true`, but like in Ruby, that's
kinda silly. Rust gives us `loop` to loop forever:

~~~ {.rust}
    loop {
        println("HELLO")
    }
~~~

Obviously you don't want to actually run that. You can use `break` to
break out of the loop:

~~~ {.rust}
    let mut i = 0;
    loop {
        i += 1;
        if i == 5 { break; }
        println("hi");
    }
~~~

This will print `"hi"` five times. You're going to want to do this,
because if someone mis-types a number, we don't want to count it against
them: we should just ask them to put in another number.

Random Number Generation
------------------------

Random number generation isn't too bad:

~~~ {.rust}
    use std::rand;

    fn main() {
        let n = rand::random::<int>();
        println(n.to_str());
    }
~~~

This will print out a different number each time you run it. But you'll
get biiiiiiig numbers. If we want 1-100, of course we have to do this:

~~~ {.rust}
	use std::rand;

    fn main() {
        let r = rand::rng().gen_int_range(0, 100);
        println(r.to_str());
    }
~~~

One issue with this: We can get negatives too. `abs` to the rescue!!!:

~~~ {.rust}
	use std::rand;
    use std::num::abs;

    fn main() {
        let r: int = abs(rand::rng().gen_int_range(0, 100));
        println(r.to_str());
    }
~~~

This will get us a random number between 1 and 100.

Okay! You should have all the tools you need to implement the guessing
game. Have it it. I'm starting... now.

My version
----------

Okay! That took me... about half an hour. Maybe 45 minutes. I decided to use
some pointer stuff... I thought it was a little awkward, though.  After asking
on IRC, 'strcat' gave me this version:

~~~ {.rust}
    use std::io;
    use std::rand;

    fn generate_secret_number() -> int {
        (rand::random::<int>() % 100).abs() + 1
    }

    fn process_guess(secret:int, guess: int) -> bool {
        println(format!("You guessed: {:d}", guess));

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

        for round in range(0, 5) {
            println(format!("Guess \\#{:d}", round));

            let input = io::stdin().read_line();

            match from_str::<int>(input) {
                Some(number) => {
                    if process_guess(secret, number) { break; }
                }
                None         => println("Hey, put in a number.")
            }
        }

        println("Done!");
    }
~~~

Conclusion
----------

I'm pretty sure at this point we have basically everything I was able to
do as a child when programming stuff. You know enough of Rust now to be
able to make silly little games and scripts. This is obviously neat, but
from this point on, it's more about libraries, style, and solving things
in an idiomatic way than it is learning syntax. Of course, this was not
a complete introduction to the language, but this is the end of the
'beginner level' stuff. You should have a basic idea of how to write
many programs by this point. Pick a few projects, try them out.

We've got one more bit to go: Making your own packages.
