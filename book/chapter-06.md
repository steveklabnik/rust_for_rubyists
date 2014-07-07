Tasks in Rust
=============

One of the things that Rust is super good at is concurrency. In order to
understand Rust's strengths, you have to understand its approach to
concurrency, and then its approach to memory.

Tasks
-----

The fundamental unit of computation in Rust is called a 'task.' Tasks are like
threads, but you can choose the low-level details of how they operate. Rust now
supports both 1:1 scheduled and N:M scheduled threads. Rust uses 1:1 threads by
default.  The details of what _exactly_ that means are out of the scope of this
tutorial, but the [Wikipedia
page](http://en.wikipedia.org/wiki/Thread_%28computing%29) has a good overview.

Here's some code that prints "Hello" 500 times:

~~~ {.rust}
fn main() {
    for num in range(0u, 500) {
        println!("Hello");
    }
}
~~~

You may remember this from earlier. This loops 500 times, printing
"Hello." Now let's make it roflscale with tasks:

~~~ {.rust}
fn main() {
    for num in range(0u, 500) {
        spawn(proc() {
            println!("Hello");
        });
    }
}
~~~

That's it! We spin up 500 tasks that print stuff. If you inspect your
output, you can tell it's working:

    Hello
    HelloHello

    Hello

Ha! Printing to the screen is obviously something that tasks can step
over each other with (if you're curious, it's because it is printing the
string and the newline separately. Sometimes, another task gets to print
its string before this task prints its newline). But the vast majority
of things aren't like that. Let's take a look at the type signature of
`spawn`:

~~~ {.rust}
fn spawn(f: proc())
~~~

Spawn is a function that takes a proc: a closure that can only be run once.
This means that Rust can do what it wants, moving the closure to another task,
or other optimizations. The details aren't particularly important at this
stage, and Rust will be undergoing some reform with regards to closures soon,
so just think of it as a closure, and that's good enough.

Pipes, Channels, and Ports
--------------------------

If our tasks are 100% isolated, they wouldn't be that useful: we need some kind
of communication between tasks in order to get back useful results. We can
communicate between tasks with pipes. Pipes have two ends: a channel that sends
info down the pipe, and a port that receives info. If you've used these
concepts in other languages, Rust's are similar, except that Rust's are
explicitly typed. Some implementations of this pattern in other languages do
not make this distinction. Otherwise, they're very similar.

Here's an example of a task that sends us back a 10:

~~~ {.rust}
fn main() {
    let (chan, port) = channel();

    spawn(proc() {
        chan.send(10u);
    });

    println!("{:s}", port.recv().to_str());
}
~~~

The `channel` function, imported by the prelude, creates both sides of this
pipe. You can imagine that instead of sending 10, we might be doing some sort
of complex calculation. It could be doing that work in the background while we
did more important things.

What about that `chan.send` bit? Well, the task captures the `chan`
variable we set up before, so it's just matter of using it. This is
similar to Ruby's blocks:

~~~ {.ruby}
foo = 10
2.times do
  puts foo
end
~~~

This is really only one-way transit, though: what if we want to
communicate back and forth? Setting up two ports and channels each time
would be pretty annoying, so we have some standard library code for
this.

We make a function that just loops forever, gets an `int` off of the
port, and sends the number plus 1 back down the channel. In the main
function, we make a channel, send one end to a new task, and then
send it a `22`, and print out the result. Because this task is running
in the background, we can send it bunches of values:

~~~ {.rust}
fn main () {
    let (fromParentSender, fromParentReceiver) = channel();
    let (fromChildSender, fromChildReceiver) = channel();

    spawn(proc() {
        plus_one(&fromChildSender, &fromParentReceiver);
    });

    fromParentSender.send(22);
    fromParentSender.send(23);
    fromParentSender.send(24);
    fromParentSender.send(25);

    for _ in range(0u, 4) {
        let answer = fromChildReceiver.recv();
        println!("{:s}", answer.to_str());
    }
}
~~~

The `use` statement imports other modules. In this case, there's a `std::comm`
module that we'll use parts of.

Pretty simple. Our task is always waiting for work. If you run this,
you'll get some weird output at the end:

    $ rustc tasks.rs && ./tasks
    23
    24
    25
    26
    task '<unnamed>' failed at 'receiving on a closed channel', /home/steveklabnik/src/rust/src/libstd/comm/mod.rs:728


`task failed at 'receiving on closed channel'`. Basically, we quit the program
without closing our child task, and so it died when our main task (the one
running `main`) died. By default, Rust tasks are bidirectionally linked, which
means if one task fails, all of its children and parents fail too. We can fix
this for now by telling our child to die:

~~~ {.rust}
use std::comm::{channel, Sender, Receiver};

fn plus_one(sender: &Sender<int>, receiver: &Receiver<int>) {
    let mut value: int;
    loop {
        value = receiver.recv();
        sender.send(value + 1);
        if value == 0 { break; }
    }
}

fn main () {
    let (fromParentSender, fromParentReceiver) = channel();
    let (fromChildSender, fromChildReceiver) = channel();

    spawn(proc() {
        plus_one(&fromChildSender, &fromParentReceiver);
    });

    fromParentSender.send(22);
    fromParentSender.send(23);
    fromParentSender.send(24);
    fromParentSender.send(24);

    fromParentSender.send(0);

    for _ in range(0i, 4) {
        let answer = fromChildReceiver.recv();
        println!("{:s}", answer.to_str());
    }
}
~~~

Now when we send a zero, our child task terminates. If you run this,
you'll get no errors at the end. We can also change our failure mode.
Rust also provides unidirectional and unlinked failure modes as well,
but I don't want to talk about them right now. This would give you
patterns like "Spin up a management task that is bidirectionally linked
to main, but have it spin up children who are unlinked." Neato.

Rust tasks are so lightweight that you can conceivably spin up a ton of
tasks, maybe even one per entity in your system.
[Servo](https://github.com/mozilla/servo) is a prototype browser
rendering engine from Mozilla, and it spins up a **ton** of tasks.
Parallel rendering, parsing, downloading, everything.

I'm imagining that most production Rust programs will eventually have a
main that spins up some sort of global task setup, and all the work gets
done inside these tasks that communicate with each other. Like, for a
video game:

~~~ {.rust}
fn main() {

    spawn(proc() {
        player_handler();
    });

    spawn(proc() {
        world_handler();
    });

    spawn(proc() {
        rendering_handler();
    });

    spawn(proc() {
        io_handler();
    });
}
~~~

... with the associated channels, of course. This feels very Actor-y to
me. I like it.
