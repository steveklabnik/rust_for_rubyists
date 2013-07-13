{% import 'macros/ork.jinja' as ork with context %}

Tasks in Rust
=============

One of the things that Rust is super good at is concurrency. In order to
understand Rust's strengths, you have to understand its approach to
concurrency, and then its approach to memory. The two are inter-related, and
we can't keep glossing over details like "just put ``~`` in front of things"
anymore.

Tasks
-----

The fundamental unit of computation in Rust is called a 'task.' Tasks are
similar to 'lightweight' threads in Erlang or Go. Rust tasks are entirely
isolated from one another, though. They're scheduled on an M:N basis to OS
threads, so they're not quite green threads exactly, either: they'll be
parallel as well as concurrent. There can be 200k Rust tasks running on 4 OS
threads (the rust scheduler by default uses one thread per core on your
computer).

As a Rubyist, you probably don't know anything about that, so let's talk code
first, and get into what exactly all that means later. Here's some code that
prints "Hello" 100 times::

  fn main() {
      for 100.times {
          println("Hello");
      }
  }

You may remember this from earlier. This loops 100 times, printing "Hello." Now
let's make it roflscale with tasks::

  fn main() {
      for 100.times {
          do spawn {
              println("Hello");
          }
      }
  }

That's it! We spin up 100 tasks that print stuff. If you inspect your output,
you can tell it's working::

  Hello
  HelloHello

  Hello

Ha! Printing to the screen is obviously something that tasks can step over
each other with (if you're curious, it's because it is printing the string and
the newline separately. Sometimes, another task gets to print its string
before this task prints its newline). But the vast majority of things aren't
like that. Let's take a look at the type signature of ``spawn``::

  fn spawn(f: ~fn())

Spawn is a function that takes a pointer to another function (it's a higher
order function). But there's that ``~`` again. This means that the pointer is
an 'owned pointer.' We'll talk more about what exactly that means in the next
chapter, but you can infer from the name that this means that we own all of
the references to the data in this closure. Therefore, Rust can move the
function around at will and know it won't break anything. The type system has
helped us determine exactly how isolated our task actually is.

Pipes, Channels, and Ports
--------------------------

If our tasks are 100% isolated, they wouldn't be that useful: we need some
kind of communication between tasks in order to get back useful results. We can
communicate between tasks with pipes. Pipes have two ends: a channel that sends
info down the pipe, and a port that receives info. Here's an example of a
task that sends us back a 10::

  use std::pipes::{stream, Port, Chan};

  fn main() {
      let (port, chan): (Port<int>, Chan<int>) = stream();

      do spawn {
          chan.send(10);
      }

      println(port.recv().to_str());
  }

Whoa! What's that new ``use`` syntax? It's called an import, and it's similar
to Ruby's ``require``, although it doesn't actually pull in new source files,
it just brings names into scope (you'll see Rust's equivalent to ``require``
later). The braces are any easy way to import multiple things without having to
repeat what you're importing from. Without it, you would need::

  use std::pipes::stream;
  use std::pipes::Port;
  use std::pipes::Chan;

This gets unwieldy fast.

You can imagine that instead of sending 10, we might be doing some sort of
complex calculation. It could be doing that work in the background while we
did more important things.

What about that ``chan.send`` bit? Well, the task captures the ``chan``
variable we set up before, so it's just matter of using it. This is similar
to Ruby's blocks::

  foo = 10
  2.times do
    puts foo
  end

This is really only one-way transit, though: what if we want to communicate
back and forth? Setting up two ports and channels each time would be pretty
annoying, so we have some standard library code for this: ``DuplexStream``::

  extern mod extra;
  use extra::comm::DuplexStream;

  fn plus_one(channel: &DuplexStream<int, int>) {
      let mut value: int;
      loop {
          value = channel.recv();
          channel.send(value + 1);
      }
  }

  fn main() {
      let (from_child, to_child) = DuplexStream();

      do spawn {
          plus_one(&to_child);
      };

      from_child.send(22);

      let twenty_three = from_child.recv();
      println(twenty_three.to_str());
  }

What's this ``extern mod extra`` madness? Well, that's how we ``link`` to
external libraries. If you've used C or C++ before, you know what this means.
If you haven't, it's essentially how you declare that your program uses a
certain dynamic library (``.dll`` on Windows, ``.dylib`` on OS X, and ``.so``
on other Unix systems). ``extra`` is part of Rust itself, it includes extras
as compared to ``std`` (which is automatically included in every program),
such as JSON parsing, networking, and data structures. See
http://static.rust-lang.org/doc/0.7/extra/index.html for more.

We make a function that just loops forever, gets an ``int`` off of the port,
and sends the number plus 1 back down the channel. In the main function, we
make a ``DuplexStream``, send one end to a new task, and then send it a
``22``, and print out the result. Because this task is running in the
background, we can send it bunches of values::

  fn main() {
      let (from_child, to_child) = DuplexStream();

      do spawn {
          plus_one(&to_child);
      };

      from_child.send(22);
      from_child.send(23);
      from_child.send(24);
      from_child.send(25);

      for 4.times {
          let answer = from_child.recv();
          println(answer.to_str());
      }
  }


Pretty simple. Our task is always waiting for work. If you run this, you'll get
some weird output at the end::

  $ rust run tasks.rs
  23
  24
  25
  26
  rust: task failed at 'connection closed', /build/src/rust-0.7/src/libstd/option.rs:307
  rust: domain main @0x7f79c4206830 root task failed

``task failed at 'connection closed'``. Basically, we quit the program without
closing our child task, and so it died when our main task (the one running
``main``) died. By default, Rust tasks are bidirectionally linked, which means
if one task fails, all of its children and parents fail too.  We can fix this
for now by telling our child to die::

  extern mod extra;
  use extra::comm::DuplexStream;

  fn plus_one(channel: &DuplexStream<int, int>) {
      let mut value: int;
      loop {
          value = channel.recv();
          channel.send(value + 1);
          if value == 0 { break; }
      }
  }

  fn main() {
    let (from_child, to_child) = DuplexStream();

    do spawn {
        plus_one(&to_child);
    };

    from_child.send(22);
    from_child.send(23);
    from_child.send(24);
    from_child.send(25);
    from_child.send(0);

    for 4.times {
        let answer = from_child.recv();
        println(answer.to_str());
    }
  }

Now when we send a zero, our child task terminates. If you run this, you'll
get no errors at the end. We can also change our failure mode. Rust also
provides unidirectional and unlinked failure modes as well, but I don't want to
talk about them right now. This would give you patterns like "Spin up a
management task that is bidirectionally linked to main, but have it spin up
children who are unlinked." Neato.

Rust tasks are so lightweight that you can conceivably spin up a ton of tasks,
maybe even one per entity in your system. Servo_ is a prototype browser
rendering engine from Mozilla, and it spins up a **ton** of tasks. Parallel
rendering, parsing, downloading, everything.

I'm imagining that most production Rust programs will eventually have a main
that spins up some sort of global task setup, and all the work gets done
inside these tasks that communicate with each other. Like, for a video game::

  fn main() {

    do spawn {
      player_handler();
    }

    do spawn {
      world_handler();
    }

    do spawn {
      rendering_handler();
    }

    do spawn {
      io_handler();
    }
  }

... with the associated channels, of course. This feels very Actor-y to me. I
like it. In fact, someone *is* working on an Actor library_! We'll see how
these kinds of things develop as Rust moves forward.

.. _Servo: https://github.com/mozilla/servo
.. _library: http://www.reddit.com/r/rust/comments/1i3c15/experimental_actor_library_in_rust/
