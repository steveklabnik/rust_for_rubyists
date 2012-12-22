{% import 'macros/ork.jinja' as ork with context %}

Installing Rust
===============

Most Rubyists use OSX, and I haven't personally installed Rust on other
platforms yet, so I'm just covering OSX for now. I'll add Windows and Linux
instructions once I've tried them out myself.

The easiest way to get Rust going is Homebrew, which you probably already use.
There's one tricky bit, though: Their Rust is out of date, which is acceptable
because a new version of Rust just came out yesterday. It's easy enough to
modify though:

```
$ brew edit rust
```

This will open up the Formula to build Rust. Change the `url` and `sha256`
lines to this:

```
url 'http://dl.rust-lang.org/dist/rust-0.5.tar.gz'                            
sha256 'd326d22707f0562d669c11efbc33ae812ddbf76ab78f07087fc5beb095a8928a'     
```

Then, install it:

```
$ brew install rust
```

If you don't use Homebrew, install it. Seriously. But if you **MUST** compile
Rust yourself, it does the Standard Unix Thing:

```
$ curl -O http://dl.rust-lang.org/dist/rust-0.5.tar.gz
$ tar -xzf rust-0.5.tar.gz
$ cd rust-0.5
$ ./configure
$ make
$ sudo make install
```

The 0.5 release of Rust has broken the Rust REPL, so a 0.5.1 is expected
shortly.

I'll try to keep these instructions up to date, but it's easy enough to find
and install the latest version; just change these two lines. The version I show
you above is the latst version I tested the code on; The language has settled
down by now, but Rust 0.6.x is expected to have a more final syntax, so some
things will change.

If you run

```
$ rustc
```

and it spits out a bunch of help information, you're good to go with Rust.
