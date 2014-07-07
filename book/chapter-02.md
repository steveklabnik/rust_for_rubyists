Installing Rust
===============

Binary installers
-----------------

The Rust project provides official binary installers. You can get both releases
and nightlies. Binary installers are the fastest and easiest way to get going
with Rust. Because Rust is written in Rust, compiling the Rust compiler actually
entails compiling it three times! This means it's quite slow. But a binary install
should be snappy!

Rust now has a [lovely downloads page](http://www.rust-lang.org/install.html),
so I recommend you just go check that out and download the proper version.

Note that this book has been tested with Rust 0.11, and so if you use the latest
nightly, something may have changed.

From Source
-----------

You will probably build the nightly version if you build from source, so
be ready for some bugs in the code samples. This book was written for 0.11.

The [Rust README](https://github.com/rust-lang/rust#building-from-source)
has great instructions for building form source. Just got follow their
instructions!

### Future Proofing

The version this book is written for is 0.11. While the language itself is
pretty stable, things like the standard library and some major subsystems are
being revised. I'll be tweaking it with every new release.

If you run

    $ rustc

and it spits out a bunch of help information, you're good to go with
Rust.
