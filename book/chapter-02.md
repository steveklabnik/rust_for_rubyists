Installing Rust
===============

Binary installers
-----------------

The Rust project provides official binary installers. You can get both releases
and nightlies. Binary installers are the fastest and easiest way to get going
with Rust. Because Rust is written in Rust, compiling the Rust compiler actually
entails compiling it three times! This means it's quite slow. But a binary install
should be snappy!

Pick your OS and your 32 or 64 bit variant.

### Linux

* http://static.rust-lang.org/dist/rust-0.10-x86_64-unknown-linux-gnu.tar.gz
* http://static.rust-lang.org/dist/rust-0.10-i686-unknown-linux-gnu.tar.gz

You'll have to run `install.sh` from inside the unzipped package.

### Ubuntu (and variants)

If you're using ubuntu, you can also install it through a
[PPA](https://launchpad.net/~hansjorg/+archive/rust):

    $ sudo apt-add-repository ppa:hansjorg/rust
    $ sudo apt-get update
    $ sudo apt-get install rust-0.10
    
You can also install the nightly build to try out changes as they're made:

    $ sudo apt-get install rust-nightly
    
And switch between them with:

    $ sudo update-alternatives --config rustc

### Mac

* http://static.rust-lang.org/dist/rust-0.10-x86_64-apple-darwin.tar.gz
* http://static.rust-lang.org/dist/rust-0.10-i686-apple-darwin.tar.gz

This should be a `.pkg` file, which you can double click to install.

### Windows (32-bit)

* http://static.rust-lang.org/dist/rust-0.10-install.exe

Double clicking an `.exe` installer should be old hat to Windows folks!

From Source
-----------

Some people prefer to build from source. It's not hard, but it does take a while!

### Mac OS X

The easiest way to get Rust going is Homebrew, which you probably
already use. Just do this:

    $ brew install rust

If you don't use Homebrew, install it. Seriously.

(Note: If you're reading this close to release, Homebrew may not have 0.10 yet,
you can use brew install --HEAD rust to get master, which will be close.)

### Linux

I personally use Linux, and Rust works quite well on it. Rust does the Standard
Unix Thing.

    $ curl -O http://static.rust-lang.org/dist/rust-0.10.tar.gz
    $ tar -xzf rust-0.10.tar.gz
    $ cd rust-0.10
    $ ./configure
    $ make
    $ sudo make install

Most package managers I've checked out either have no package or a really old
package, so you'll probably want to just install from source.

### Windows

See instructions on the
[wiki](https://github.com/mozilla/rust/wiki/Note-getting-started-developing-Rust#windows).
Overall, Rust wants to have strong Windows support, but some of it is in flux.
Don't be afraid to hop into [the Rust
IRC](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) and ask
for help.

### Future Proofing

The version this book is written for is 0.10. While the language itself is
pretty stable, things like the standard library and some major subsystems are
being revised. I'll be tweaking it with every new release.

If you run

    $ rustc

and it spits out a bunch of help information, you're good to go with
Rust.
