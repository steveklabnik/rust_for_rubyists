Installing Rust
===============

Most Rubyists use OS X, and Rust is pretty easy to get going on it:

Mac OS X
--------

The easiest way to get Rust going is Homebrew, which you probably
already use. Just do this:

    $ brew install rust

If you don't use Homebrew, install it. Seriously.

(Note: If you're reading this close to release, Homebrew may not have 0.9 yet,
you can use brew install --HEAD rust to get master, which will be close.)

Linux
-----

I personally use Linux, and Rust works quite well on it. Rust does the Standard
Unix Thing.

    $ curl -O http://static.rust-lang.org/dist/rust-0.9.tar.gz
    $ tar -xzf rust-0.9.tar.gz
    $ cd rust-0.9
    $ ./configure
    $ make
    $ sudo make install

Most package managers I've checked out either have no package or a really old
package, so you'll probably want to just install from source.

Windows
-------

See instructions on the
[wiki](https://github.com/mozilla/rust/wiki/Note-getting-started-developing-Rust#windows).
Overall, Rust wants to have strong Windows support, but some of it is in flux,
and it was decided that the 0.8 release would be a bit wonky. Don't be afraid to 
hop into [the Rust
IRC](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) and
ask for help.

Future Proofing
---------------

The version this book is written for is 0.9. While the language itself is pretty
stable, things like the standard library and some major subsystems are being
revised. I'll be tweaking it with every new release.

If you run

    $ rustc

and it spits out a bunch of help information, you're good to go with
Rust.
