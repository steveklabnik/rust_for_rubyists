{% import 'macros/ork.jinja' as ork with context %}

Traits and Generics
===================

Now that we understand a type that's sorta generic, vectors, we can talk about
how generic functions work. Then, we can use traits to make functions that work
on generic monsters.

Writing functions that work with vectors
----------------------------------------

Because you're still getting used to Rust code, let's implement two versions of
a method that print everything in a vector, and then refactor that into the
generic form.

Let's do an exercise. You have this code::

  fn main() {
      let vec = [1,2,3];

      print_vec(vec);
  }

Implement ``print_vec`` so that it puts out ``1 2 3`` with newlines between
them. Hint: You can write 'I want an array of ints' with ``&[int]``. Remember
how functions often use borrowed pointers?

I'll wait.

Done? I got this::

  fn print_vec(v: &[int]) {

    for v.each |&i| {
      println(int::to_str(i))
    }
  }

  fn main() {
    let vec = [1,2,3];

    print_vec(vec);
  }

Pretty straightforward. We take a slice (remember, 'borrowed vector' ==
'slice') of ints, get a borrowed pointer to each of them, and print them out.

Round two: Implement this::

  fn main() {
      let vec = [1,2,3];

      print_vec(vec);

      let str_vec = [~"hey", ~"there", ~"yo"];

      print_vec_str(str_vec);
  }

You'll often be seeing owned boxes with strings. Go ahead. You can do it!

I got this::

  fn print_vec(v: &[int]) {
      for v.each |&i| {
          println(int::to_str(i))
      }
  }

  fn print_vec_str(v: &[~str]) {
      for v.each |&i| {
          println(i)
      }
  }

  fn main() {
      let vec = [1,2,3];

      print_vec(vec);

      let str_vec = [~"hey", ~"there", ~"yo"];

      print_vec_str(str_vec);
  }

You'll notice we had to declare what type of ``str`` we had. See, strings
are actually implemented as vectors of characters, so while they are sorta a
type, you can't have just ``str`` as a type. You gotta say what kind of box
they're in.

Okay, obviously, this situation sucks! What can we do? Well, the first thing
is that we don't have the same method body. We're doing different things to
convert our arguments to a string. Here's the answer::

  fn print_vec(v: &[int]) {

    for v.each |&i| {
        println(i.to_str())
    }
  }

  fn print_vec_str(v: &[~str]) {

    for v.each |&i| {
        println(i.to_str())
    }
  }

  fn main() {
      let vec = [1,2,3];

      print_vec(vec);

      let str_vec = [~"hey", ~"there", ~"yo"];

      print_vec_str(str_vec);
  }

Now that you know about methods, you can see how this works: there's a method
on strings and on vectors called ``to_str``, and it converts it to a string.
This is much nicer than our ``int::str`` calls from before, in my opinion.
And now that we have the same method body, our types are almost the same...

Let's fix that::

  fn print_vec<T>(v: &[T]) {
      for v.each |&i| {
          println(i.to_str())
      }
  }

  fn main() {
      let vec = [1,2,3];

      print_vec(vec);

      let str_vec = [~"hey", ~"there", ~"yo"];

      print_vec(str_vec);
  }

This won't compile, but it is closer. Let's examine that signature more
closely.

* ``<T>`` says that we're going to be making this function polymorphic over the
  type T.
* We then use it later to say we take a borrowed pointer of a vector
  of ``T`` s, ``&[T]``

If you try to compile this, you'll get an error::

  $ make
  rustc fizzbuzz.rs
  fizzbuzz.rs:4:16: 4:27 error: type `'a` does not implement any method in scope named `to_str`
  fizzbuzz.rs:4     io::println(i.to_str())
                                ^~~~~~~~~~~
  error: aborting due to previous error
  make: *** [build] Error 101

This is a problem. Our generic type T does not have any restrictions on what
kind of thing it is, which means we can't guarantee that we'll get something
that has the ``to_str`` method defined on it.

For that, we need Traits.

Traits
------

This **will** work::

  fn print_vec<T: ToStr>(v: &[T]) {
      for v.each |&i| {
          println(i.to_str())
      }
  }

  fn main() {
      let vec = [1,2,3];

      print_vec(vec);

      let str_vec = [~"hey", ~"there", ~"yo"];

      print_vec(str_vec);
  }

The ``<T: ToStr>`` says: "We take any type ``T`` that implements the ``ToStr``
trait.

Traits are sort of like 'static duck typing' or 'structural typing.' We get
away with this in Ruby by just trusting the code we write, and for most of it,
it just works out. Think about this::

  def print_each(arr)
    arr.each do |i|
      puts i
    end
  end

We trust that this will always work, because ``Object`` implements ``#to_str``.
But if we had this::

  def print_each(arr)
    arr.each do |i|
      puts i + 1
    end
  end

We have an implicit type here: ``arr`` must contain things that
``respond_to?(:+)``. In many ways, Rust is sorta like::

  def print_each(arr)
    assert arr.respond_to?(:+)

    arr.each do |i|
      puts i + 1
    end
  end

But it happens at compile time, not run time.

Now, I've never written code where I felt the need to check for a
``NoMethodError`` or ``TypeError``, as you'd get in Ruby::

  irb(main):007:0> print_each(["a","b","c"])
  TypeError: can't convert Fixnum into String
    from (irb):3:in `+'
    from (irb):3:in `block in print_each'
    from (irb):2:in `each'
    from (irb):2:in `print_each'
    from (irb):7
    from /usr/local/ruby-1.9.3-p327/bin/irb:12:in `<main>'

But I think that safety is the wrong way to look at this kind of static typing.
The right way to look at it is that by giving the compiler more information
about our code, it can make certain optimizations. Check this out::

  $ cat fizzbuzz.rs
  fn print_vec<T: ToStr>(v: &[T]) {
      for v.each |&i| {
          println(i.to_str())
      }
  }

  fn main() {
    let vec = [1,2,3];

    print_vec(vec);

    let str_vec = [~"hey", ~"there", ~"yo"];

    print_vec(str_vec);
  }

  steve at thoth in ~/tmp
  $ make 
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz
  1
  2
  3
  hey
  there
  yo

  steve at thoth in ~/tmp
  $ nm fizzbuzz | grep vec
  00000001000010e0 t __ZN14print_vec_183116_7451ef3beba84213_00E
  0000000100001930 t __ZN14print_vec_18314anon12expr_fn_1887E
  0000000100001c70 t __ZN14print_vec_191917_3a74ff88f1eb6fd73_00E
  0000000100002290 t __ZN14print_vec_19194anon12expr_fn_1945E
  0000000100001150 t __ZN3vec14__extensions__9each_183417_4665ed5b2714d02e3_00E
  0000000100001890 t __ZN3vec14__extensions__9each_18344anon12expr_fn_1885E
  0000000100001ce0 t __ZN3vec14__extensions__9each_192217_e0ecf3d9b9b0715e3_00E
  00000001000021f0 t __ZN3vec14__extensions__9each_19224anon12expr_fn_1943E
  00000001000012a0 t __ZN3vec15as_imm_buf_184017_fd547453b8ba742f3_00E
  0000000100001e30 t __ZN3vec15as_imm_buf_192815_373c391b86ef533_00E
  0000000100001200 t __ZN3vec9each_183717_9abf2ac654d785153_00E
  0000000100001550 t __ZN3vec9each_18374anon12expr_fn_1865E
  0000000100001d90 t __ZN3vec9each_192516_19945ee2203b48c3_00E
  0000000100002010 t __ZN3vec9each_19254anon12expr_fn_1935E
  0000000100001670 t __ZN4cast22copy_lifetime_vec_186717_8dfcb0f579fd27b63_00E
  0000000100002130 t __ZN4cast22copy_lifetime_vec_193717_7ef7e3f59d8b71db3_00E

  steve at thoth in ~/tmp
  $ mvim fizzbuzz.rs

  steve at thoth in ~/tmp
  $ cat fizzbuzz.rs
  fn print_vec<T: ToStr>(v: &[T]) {
      for v.each |&i| {
          println(i.to_str())
      }
  }

  fn main() {
      let vec = [1,2,3];

      print_vec(vec);
  }

  steve at thoth in ~/tmp
  $ rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)

  $ nm fizzbuzz | grep vec
  0000000100000fe0 t __ZN14print_vec_182716_7451ef3beba84213_00E
  0000000100001830 t __ZN14print_vec_18274anon12expr_fn_1883E
  0000000100001050 t __ZN3vec14__extensions__9each_183017_4665ed5b2714d02e3_00E
  0000000100001790 t __ZN3vec14__extensions__9each_18304anon12expr_fn_1881E
  00000001000011a0 t __ZN3vec15as_imm_buf_183617_fd547453b8ba742f3_00E
  0000000100001100 t __ZN3vec9each_183317_9abf2ac654d785153_00E
  0000000100001450 t __ZN3vec9each_18334anon12expr_fn_1861E
  0000000100001570 t __ZN4cast22copy_lifetime_vec_186317_8dfcb0f579fd27b63_00E

Okay. So the first time we have our code, we have two calls to ``print_vec``,
one for a vector of strings and one for a vector of ints. The call to ``nm``...

Oh wait, I haven't told you about ``nm``!

A diversion about nm
--------------------

Here's what my manpage says::

  $ man nm

  NAME
         nm - display name list (symbol table)

  SYNOPSIS
         nm  [  -agnoprumxjlfPA  [  s segname sectname ]] [ - ] [ -t format ] [[
         -arch arch_flag ]...] [ file ... ]

  DESCRIPTION
         Nm displays the name list (symbol table) of each  object  file  in  the
         argument list. 

Cool! You've never had to think about symbol tables before, so let's talk about
them.

When your compiler compiles something, you get an 'object file' out of it.
This is the binary that you run: ``rustc fizzbuzz.rs`` produces ``fizzbuzz``.
This object file will contain a list of ``symbols`` and where they exist in 
memory. This matters when we want to write two bits of code that work together:
If my library exposes a function called ``my_function``, and you want to use
it, the compiler needs to know where to find ``my_function`` in my library's
code. The compiler 'mangles' the names to fit its own scheme. This is called an
"ABI", or application binary interface. Have you ever seen this::

  /Users/Steve/.rvm/rubies/ruby-1.9.3-p286/lib/ruby/1.9.1

And wondered why that 1.9.1 is there? That's because Ruby 1.9.3 and Ruby 1.9.1
both share the same ABI, so gems that are linked against 1.9.1 can also be
used with 1.9.3. They use the same scheme to generate symbols.

Anyway, ``nm`` can show us this information. The first column is the location
in memory, the second is the (mangled) name::

  0000000100001bb8 S _rust_abi_version

That's a fun, recursive symbol ;) Anyway, we can examine what symbols Rust
exports to see some information about our executable, that's my intention with
``nm`` in this case.

Back to our regularly scheduled investigation
---------------------------------------------

Here's the important part of the two outputs of nm::

  00000001000010e0 t __ZN14print_vec_183116_7451ef3beba84213_00E
  0000000100001930 t __ZN14print_vec_18314anon12expr_fn_1887E
  0000000100001c70 t __ZN14print_vec_191917_3a74ff88f1eb6fd73_00E
  0000000100002290 t __ZN14print_vec_19194anon12expr_fn_1945E

and::

  0000000100000fe0 t __ZN14print_vec_182716_7451ef3beba84213_00E
  0000000100001830 t __ZN14print_vec_18274anon12expr_fn_1883E

See how they both have ``print_vec``? These are the functions we made. And
without even knowing what's happening, you can see the difference: in the
version of our code where we call ``print_vec`` on strings and ints, we have
two versions of the function, and on the version where we just call it on ints,
we have one version.

Neat! We get specialized versions, but only specialized for the types we
actually use. No generating code that's useless. This process is called
'monomorphism,' which basically means we take one thing (mono) and change it
(morphism) into other things. To simplify, the compiler takes this code::

  fn print_vec<T: ToStr>(v: &[T]) {
      for v.each |&i| {
          println(i.to_str())
      }
  }

  fn main() {
      let vec = [1,2,3];

      print_vec(vec);

      let str_vec = [~"hey", ~"there", ~"yo"];

      print_vec(str_vec);
  }

And turns it into::

  fn print_vec_str(v: &[~str]) {
      for v.each |&i| {
          println(i.to_str())
      }
  }

  fn print_vec_int(v: &[int]) {
      for v.each |&i| {
          println(i.to_str())
      }
  }

  fn main() {
      let vec = [1,2,3];

      print_vec_int(vec);

      let str_vec = [~"hey", ~"there", ~"yo"];

      print_vec_str(str_vec);
  }

Complete with changing the calls at each call site to call the special version
of the function. We call this 'static dispatch,' as opposed to the 'dynamic
dispatch' that'd happen at runtime.

These are the kinds of optimizations that we get with static typing. Neat! I
will say that there are efforts to bring this kind of optimization into
dynamically typed languages as well, through analyzing the call site. So, for
example::

  def foo(arg)
    puts arg
  end

If we call ``foo`` with a ``String`` ``arg`` a bunch of times in a row, the
interpreter will JIT compile a version of ``foo`` specialized for ``Strings``,
and then replace the call site with something like::

  if arg.kind_of? String
    __super_optimized_foo_string(arg)
  else
    foo(arg)
  end

This would give you the same benefit, without the human typing. Not just that,
but a sufficiently smart runtime would be able to actually determine more
complex situations that a person may not. And, maybe after, say, 1000 calls
with a String, just remove the check entirely.

Anyway.

Making our own Traits
---------------------

We want all of our monsters to implement ``attack``. So let's make ``Monster``
a Trait. The syntax looks like this::

  trait Monster {
      fn attack(&self);
  }

This says that the ``Monster`` trait guarantees we have one method available
on any type that implements the trait, ``attack``. Here's how we make one::

  trait Monster {
      fn attack(&self);
  }

  struct IndustrialRaverMonkey {
      strength: int
  }

  impl IndustrialRaverMonkey: Monster {
      fn attack(&self) {
          println(fmt!("The monkey attacks for %d.", self.strength))
      }
  }

  fn main() {
      let monkey = IndustrialRaverMonkey {strength:35};

      monkey.attack();
  }

Now we're cooking with gas! Remember our old implementation?::

  impl Monster {
      fn attack(&self) {
          match *self {
              ScubaArgentine(l, s, c, w) => println(fmt!("The monster attacks for %d damage.", w)),
              IndustrialRaverMonkey(l, s, c, w) => println(fmt!("The monster attacks for %d damage.", w))
          }
      }
  }

Ugh. This is way better. No de-structuring on types. We can write an
implementation for absolutely anything::

  trait Monster {
      fn attack(&self);
  }

  struct IndustrialRaverMonkey {
      strength: int
  }

  impl Monster for IndustrialRaverMonkey {
      fn attack(&self) {
          println(fmt!("The monkey attacks for %d.", self.strength))
      }
  }

  impl Monster for int {
      fn attack(&self) {
          println(fmt!("The int attacks for %d.", *self))
      }
  }

  fn main() {
      let monkey = IndustrialRaverMonkey {strength:35};
      monkey.attack();

      let i = 10;
      i.attack();
  }

Heh. Check it::

  $ make
  rustc fizzbuzz.rs
  warning: no debug symbols in executable (-arch x86_64)
  ./fizzbuzz
  The monkey attacks for 35.
  The int attacks for 10.

Amusing.

One last issue: Due to the way Rust is right now, if you want a vector of
things as a trait, you need to do this::

  let dwemthys_vector: @[@Monster] = @[monkey as @Monster, angel as @Monster, tentacle as @Monster, deer as @Monster, cyclist as @Monster, dragon as @Monster];

Get that? We make a vector that's a shared pointer of shared pointers to
``Monster`` s. We have to declare that we want them that way by saying ``as
@Monster``, which is awkward. I've been reassured that this will hopefully
disappear in future versions of Rust, but you gotta do it for now.

Okay, exercise: Make six different monsters, and create a vector with all of
them in it. Then write a method that takes the vector, and prints out all of
the monsters and their stats.

I'll wait. It took me a little while to write this: this is the hardest part of
the book so far. Work through it; it'll be painful. Don't be afraid to ask for
help. I had to ask `the rust IRC`_ for help once while doing it. They're
friendly, don't worry.

Done? Here's mine::

  trait Monster {
      fn attack(&self);
      fn new() -> Self;
  }

  struct IndustrialRaverMonkey {
      life: int,
      strength: int,
      charisma: int,
      weapon: int,
  }

  struct DwarvenAngel {
      life: int,
      strength: int,
      charisma: int, 
      weapon: int,
  }

  struct AssistantViceTentacleAndOmbudsman {
      life: int,
      strength: int,
      charisma: int, 
      weapon: int,
  }

  struct TeethDeer {
      life: int,
      strength: int,
      charisma: int,
      weapon: int,
  }

  struct IntrepidDecomposedCyclist {
      life: int,
      strength: int,
      charisma: int, 
      weapon: int,
  }

  struct Dragon {
      life: int,
      strength: int,
      charisma: int, 
      weapon: int,
  }

  impl Monster for IndustrialRaverMonkey {
      fn attack(&self) {
          println(fmt!("The monkey attacks for %d.", self.strength))
      }

      fn new() -> IndustrialRaverMonkey {
          IndustrialRaverMonkey {life: 46, strength: 35, charisma: 91, weapon: 2}
      }
  }

  impl Monster for DwarvenAngel {
      fn attack(&self) {
          println(fmt!("The angel attacks for %d.", self.strength))
      }
      fn new() -> DwarvenAngel {
          DwarvenAngel {life: 540, strength: 6, charisma: 144, weapon: 50}
      }
  }

  impl Monster for AssistantViceTentacleAndOmbudsman {
      fn attack(&self) {
          println(fmt!("The tentacle attacks for %d.", self.strength))
      }
      fn new() -> AssistantViceTentacleAndOmbudsman {
          AssistantViceTentacleAndOmbudsman {life: 320, strength: 6, charisma: 144, weapon: 50}
      }
  }

  impl Monster for TeethDeer {
      fn attack(&self) {
          println(fmt!("The deer attacks for %d.", self.strength))
      }
      fn new() -> TeethDeer {
          TeethDeer {life: 655, strength: 192, charisma: 19, weapon: 109}
      }
  }

  impl Monster for IntrepidDecomposedCyclist {
      fn attack(&self) {
          println(fmt!("The cyclist attacks for %d.", self.strength))
      }
      fn new() -> IntrepidDecomposedCyclist {
          IntrepidDecomposedCyclist {life: 901, strength: 560, charisma: 422, weapon: 105}
      }
  }

  impl Monster for Dragon {
      fn attack(&self) {
          println(fmt!("The dragon attacks for %d.", self.strength))
      }
      fn new() -> Dragon {
          Dragon {life: 1340, strength: 451, charisma: 1020, weapon: 939}
      }
  }

  fn monsters_attack(monsters: &[@Monster]) {
      for monsters.each |monster| {
          monster.attack();
      }
  }

  fn main() {
      let monkey: @IndustrialRaverMonkey               = @Monster::new();
      let angel: @DwarvenAngel                         = @Monster::new();
      let tentacle: @AssistantViceTentacleAndOmbudsman = @Monster::new();
      let deer: @TeethDeer                             = @Monster::new();
      let cyclist: @IntrepidDecomposedCyclist          = @Monster::new();
      let dragon: @Dragon                              = @Monster::new();

      let dwemthys_vector: @[@Monster] = @[monkey as @Monster, angel as @Monster, tentacle as @Monster, deer as @Monster, cyclist as @Monster, dragon as @Monster];

      monsters_attack(dwemthys_vector);
  }

Congrats! You've mastered Traits. They're pretty awesome, right?

.. _The Rust IRC: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
