Why care about Rust?
====================

You already write software in Ruby. It pays your bills. You enjoy it.
Why should you care about Rust?

Alan Perlis once said:

> A language that doesn't affect the way you think about programming is
> not worth knowing.

Let's think about Ruby for a minute: what's its biggest weakness? For
me, it's these things:

-   Concurrency
-   Safety guarantees
-   Lots of mutable state
-   Only vaguely functional
-   Speed
-   Complexity. (Smalltalk's semantics fit on an index card)
-   Documentation
-   nil

What's awesome about Ruby?

-   Blocks
-   Vaguely functional
-   Syntax is pretty easy
-   Focus on developer happiness
-   Get up and running quickly
-   Dynamically typed

So we could learn a lot from a language that handles concurrency well,
has good safety guarantees, is immutable by default, and is fast and
simple. We don't want to sacrifice anonymous functions, pretty syntax,
or not making `AbstractFactoryFactoryImpls` just to get work done.

I think that that language is Rust.

Now: Rust is not perfect, by far. Its documentation is poor. It is
certainly *very* complex. Fighting with a compiler can be frustrating.
But the point is to *learn*. And using a language that's very familiar,
yet very different, can teach us a lot.

Here's "Hello World" in Rust:

~~~ {.rust}
    fn main() {
        println("Hello, world!");
    }
~~~

Here's a parallel "Hello World" in Rust:

~~~ {.rust}
    fn main() {
        for num in range(0, 10) {
            spawn(proc() {
                let greeting_message = "Hello?";
                println!("{}", greeting_message);
            });
        }
    }
~~~

Here's a rough port to Ruby:


~~~ {.ruby}
    10.times do
      Thread.new do
        greeting_message = "Hello?"
        puts "#{greeting_message}" # slightly unnatural ruby to match `format!` macro in rust.
      end
    end
~~~

That's it. Note the stuff that's *similar* to Ruby:

-   Variables are in `snake_case`
-   We have 'blocks' that use `{}`. No `do/end` though.
-   Variables, while statically typed, have inference, so we don't need
    to declare types

Here's some stuff that's *different*:

-   `;` s everywhere. You don't always need them, but let's put them in
    for now.
-   slightly different syntax, `fn` rather than `def`.
-   Because we have no `do/end`, we use `{}` s instead.
-   The compiler will yell at us harder if we mess up.

Oh, and:

    $ time ./hello
    ./hello 0.01s user 0.01s system 91% cpu 0.014 total

    $ time ruby hello.rb
    ruby hello.rb 0.02s user 0.01s system 95% cpu 0.026 total

Twice as fast. Yay irrelevant microbenchmarks!

Anyway, I hope you get my point: There's lots of things about Rust that
make it syntactically vaguely similar enough to Ruby that you can feel
at home, at least at first. And its strengths are some of Ruby's
greatest weaknesses. That's why I think you can learn a lot from playing
with Rust, even if you don't do it as your day job.
