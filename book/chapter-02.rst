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

  $ curl -O http://dl.rust-lang.org/dist/rust-0.6.tar.gz
  $ tar -xzf rust-0.6.tar.gz
  $ cd rust-0.6
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

The version this book is written for is 0.6. The language has largely calmed
down, so it should be pretty future-proof code. I'll be tweaking it with every
new release.

If you run

::

  $ rustc

and it spits out a bunch of help information, you're good to go with Rust.

.. _installer: http://static.rust-lang.org/dist/rust-0.6-install.exe
