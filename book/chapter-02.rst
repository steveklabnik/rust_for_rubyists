{% import 'macros/ork.jinja' as ork with context %}

Installing Rust
===============

Most Rubyists use OS X, and I haven't personally installed Rust on other
platforms yet, so I'm just covering OS X for now. I'll add Windows and Linux
instructions once I've tried them out myself.

Mac OS X
--------

The easiest way to get Rust going is Homebrew, which you probably already use.
Just do this::

  $ brew install rust

If you don't use Homebrew, install it. Seriously.

Linux
-----

Rust does the Standard Unix Thing.

::

  $ curl -O http://dl.rust-lang.org/dist/rust-0.5.tar.gz
  $ tar -xzf rust-0.5.tar.gz
  $ cd rust-0.5
  $ ./configure
  $ make
  $ sudo make install

Most package managers I've checked out either have no package or a really old
package, so you'll probably want to just install from source.

Windows
-------

I have not tried to install Rust on Windows, but I hear it works well. You can
use the installer_.

Future Proofing
---------------

The 0.5 release of Rust has broken the Rust REPL, so a 0.5.1 is expected
shortly.

I'll try to keep these instructions up to date, but it's easy enough to find
and install the latest version; just change these two lines. The version I show
you above is the latest version I tested the code on; The language has settled
down by now, but Rust 0.6.x is expected to have a more final syntax, so some
things will change.

If you run

::

  $ rustc

and it spits out a bunch of help information, you're good to go with Rust.

.. _installer: http://static.rust-lang.org/dist/rust-0.5-install.exe
