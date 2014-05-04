Traits and Generics
===================

Now that we understand a type that's sorta generic, vectors, we can talk
about how generic functions work. Then, we can use traits to make
functions that work on generic monsters.

Writing functions that work with vectors
----------------------------------------

Because you're still getting used to Rust code, let's implement two
versions of a method that print everything in a vector, and then
refactor that into the generic form.

Let's do an exercise. You have this code:

~~~ {.rust}
    fn main() {
        let vec = [1,2,3];

        print_vec(vec);
    }
~~~

Implement `print_vec` so that it puts out `1 2 3` with newlines between
them. Hint: You can write 'I want an array of ints' with `&[int]`.
Remember how functions can often use borrowed pointers?

I'll wait.

Done? I got this:

~~~ {.rust}
    use std::io::println;

    fn print_vec(v: &[int]) {
        for i in v.iter() {
            println(i.to_str())
        }
    }

    fn main() {
        let vec = [1,2,3];

        print_vec(vec);
    }
~~~

Pretty straightforward. We take a slice (remember, 'borrowed vector' ==
'slice') of ints, get a borrowed pointer to each of them, and print them
out.

Round two: Implement this:

~~~ {.rust}
    fn main() {
        let vec = [1,2,3];

        print_vec(vec);

        let str_vec = [~"hey", ~"there", ~"yo"];

        print_vec_str(str_vec);
    }
~~~

You'll often be seeing owned pointers with strings. Go ahead. You can do it!

I got this:

~~~ {.rust}
    use std::io::println;

    fn print_vec(v: &[int]) {
        for i in v.iter() {
            println(i.to_str())
        }
    }

    fn print_vec_str(v: &[~str]) {
        for i in v.iter() {
            println(*i)
        }
    }

    fn main() {
        let vec = [1,2,3];

        print_vec(vec);

        let str_vec = [~"hey", ~"there", ~"yo"];

        print_vec_str(str_vec);
    }
~~~

You'll notice we had to declare what type of `str` we had. See, strings
are actually implemented as vectors of characters (encoded in UTF-8), so
while they are sorta a type, you can't have just `str` as a type. You
gotta say `~str`.

Okay, obviously, this situation sucks! What can we do? Well, the first
thing is that we don't have the same method body. We're doing different
things to convert our arguments to a string. Here's the answer:

~~~ {.rust}
    use std::io::println;

    fn print_vec(v: &[int]) {
        for i in v.iter() {
            println!("{}", i)
        }
    }

    fn print_vec_str(v: &[~str]) {
        for i in v.iter() {
            println!("{}", i)
        }
    }

    fn main() {
        let vec = [1,2,3];

        print_vec(vec);

        let str_vec = [~"hey", ~"there", ~"yo"];

        print_vec_str(str_vec);
    }
~~~

And now that we have the same method body, our types are almost the
same... Let's fix that:

~~~ {.rust}
    fn print_vec<T>(v: &[T]) {
        for i in v.iter() {
            println!("{}", i)
        }
    }

    fn main() {
        let vec = [1,2,3];

        print_vec(vec);

        let str_vec = [~"hey", ~"there", ~"yo"];

        print_vec(str_vec);
    }
~~~

This won't compile, but it is closer. Let's examine that signature more
closely.

-   `<T>` says that we're going to be making this function polymorphic
    over the type T.
-   We then use it later to say we take a borrowed pointer of a vector
    of `T` s, `&[T]`

If you try to compile this, you'll get an error:

    $ rustc traits.rs && ./traits
    fizzbuzz.rs:5:28: 5:29 error: failed to find an implementation of trait std::fmt::Show for T
    fizzbuzz.rs:5             println!("{}", i)


This is a problem. Our generic type T does not have any restrictions on
what kind of thing it is, which means we can't guarantee that we'll get
something that has the ability to be displayed.

For that, we need Traits.

Traits
------

This **will** work:

~~~ {.rust}
    fn print_vec<T: std::fmt::Show>(v: &[T]) {
        for i in v.iter() {
            println!("{}", i)
        }
    }

    fn main() {
        let vec = [1,2,3];

        print_vec(vec);

        let str_vec = [~"hey", ~"there", ~"yo"];

        print_vec(str_vec);
    }
~~~

The `<T: std::fmt::Show>` says: "We take any type `T` that implements the `Show`
trait.

Traits are sort of like 'static duck typing' or 'structural typing.' We
get away with this in Ruby by just trusting the code we write, and for
most of it, it just works out. Think about this:

~~~ {.ruby}
    def print_each(arr)
      arr.each do |i|
        puts i
      end
    end
~~~

We trust that this will always work, because `Object` implements
`#to_s`. But if we had this:

~~~ {.ruby}
    def print_each(arr)
      arr.each do |i|
        puts i + 1
      end
    end
~~~

We have an implicit type here: `arr` must contain things that
`respond_to?(:+)`. In many ways, Rust is sorta like:

~~~ {.ruby}
    def print_each(arr)
      assert arr.respond_to?(:+)

      arr.each do |i|
        puts i + 1
      end
    end
~~~

But it happens at compile time, not run time.

Now, I've never written code where I felt the need to check for a
`NoMethodError` or `TypeError`, as you'd get in Ruby:

    irb(main):007:0> print_each(["a","b","c"])
    TypeError: can't convert Fixnum into String
      from (irb):3:in `+'
      from (irb):3:in `block in print_each'
      from (irb):2:in `each'
      from (irb):2:in `print_each'
      from (irb):7
      from /usr/local/ruby-1.9.3-p327/bin/irb:12:in `<main>'

But I think that safety is the wrong way to look at this kind of static
typing. The right way to look at it is that by giving the compiler more
information about our code, it can make certain optimizations. Check
this out:

    $ cat traits.rs
~~~ {.rust}
    fn print_vec<T: std::fmt::Show>(v: &[T]) {
        for i in v.iter() {
            println!("{}", i)
        }
    }

    fn main() {
        let vec = [1,2,3];

        print_vec(vec);

        let str_vec = [~"hey", ~"there", ~"yo"];

        print_vec(str_vec);
    }
~~~

    $ rustc traits.rs && ./traits
    1
    2
    3
    hey
    there
    yo

    $ nm -C traits~ | grep vec
    0000000000401500 t print_vec_2912::_85e5a3bc2d3e1a83::_00
    0000000000401ee0 t print_vec_2912::anon::expr_fn_2970
    0000000000404cd0 t print_vec_3218::_f1e1b4437dbb28a::_00
    0000000000405480 t print_vec_3218::anon::expr_fn_3252
    0000000000402c50 t vec::__extensions__::reserve_3030::_de1a9d6344b57ab::_00
    0000000000402d70 t vec::__extensions__::capacity_3032::_824484774e7757::_00
    0000000000404b50 t
    vec::__extensions__::push_fast_3194::_5cf6fa3bfa6090d7::_00
    0000000000404ae0 t
    vec::__extensions__::reserve_at_least_3192::_de1a9d6344b57ab::_00
    0000000000404840 t
    vec::__extensions__::reserve_no_inline_3182::_24c451fdab89623e::_00
    0000000000401c50 t vec::__extensions__::len_2959::_824484774e7757::_00
    0000000000401e80 t vec::__extensions__::len_2959::anon::expr_fn_2968
    00000000004048b0 t vec::__extensions__::len_3185::_824484774e7757::_00
    0000000000404a80 t vec::__extensions__::len_3185::anon::expr_fn_3190
    00000000004051f0 t vec::__extensions__::len_3243::_824484774e7757::_00
    0000000000405420 t vec::__extensions__::len_3243::anon::expr_fn_3250
    0000000000401a50 t vec::__extensions__::iter_2947::_d7a5bdd54e5e6f77::_00
    00000000004050a0 t vec::__extensions__::iter_3237::_55446721964a82e1::_00
    0000000000401680 t vec::__extensions__::next_2919::_5079d793a0f371c9::_00
    0000000000404e50 t vec::__extensions__::next_3224::_b423b136d356fe1d::_00
    0000000000404790 t vec::__extensions__::push_3179::_a91dd4803fb62a::_00
    0000000000401d00 t vec::as_imm_buf_2961::_caa46d7965b990b9::_00
    0000000000404970 t vec::as_imm_buf_3187::_62a416e4b98acea8::_00
    00000000004052a0 t vec::as_imm_buf_3245::_cb6b3bad8005286::_00
    0000000000401b30 t vec::raw::to_ptr_2950::_1df29a3554bbd95b::_00
    0000000000405180 t vec::raw::to_ptr_3240::_8c11f86a3948f562::_00
                     U
                     vec::rustrt::vec_reserve_shared_actual::_c688b9b8fd5bf21::_07


    $ mvim traits.rs
    ....editing...
    $ cat traits.rs
~~~ {.rust}
    fn print_vec<T: ToStr>(v: &[T]) {
        for i in v.iter() {
            println((*i).to_str())
        }
    }

    fn main() {
        let vec = [1,2,3];

        print_vec(vec);
    }
~~~

    $ rustc traits.rs && ./traits

    $ nm -C traits~ | grep vec
    00000000004012d0 t print_vec_2908::_85e5a3bc2d3e1a83::_00
    0000000000401cb0 t print_vec_2908::anon::expr_fn_2966
    0000000000402a20 t vec::__extensions__::reserve_3026::_de1a9d6344b57ab::_00
    0000000000402b40 t vec::__extensions__::capacity_3028::_824484774e7757::_00
    0000000000404920 t
    vec::__extensions__::push_fast_3190::_5cf6fa3bfa6090d7::_00
    00000000004048b0 t
    vec::__extensions__::reserve_at_least_3188::_de1a9d6344b57ab::_00
    0000000000404610 t
    vec::__extensions__::reserve_no_inline_3178::_24c451fdab89623e::_00
    0000000000401a20 t vec::__extensions__::len_2955::_824484774e7757::_00
    0000000000401c50 t vec::__extensions__::len_2955::anon::expr_fn_2964
    0000000000404680 t vec::__extensions__::len_3181::_824484774e7757::_00
    0000000000404850 t vec::__extensions__::len_3181::anon::expr_fn_3186
    0000000000401820 t vec::__extensions__::iter_2943::_d7a5bdd54e5e6f77::_00
    0000000000401450 t vec::__extensions__::next_2915::_5079d793a0f371c9::_00
    0000000000404560 t vec::__extensions__::push_3175::_a91dd4803fb62a::_00
    0000000000401ad0 t vec::as_imm_buf_2957::_caa46d7965b990b9::_00
    0000000000404740 t vec::as_imm_buf_3183::_62a416e4b98acea8::_00
    0000000000401900 t vec::raw::to_ptr_2946::_1df29a3554bbd95b::_00
                     U
                     vec::rustrt::vec_reserve_shared_actual::_c688b9b8fd5bf21::_07

Okay. So the first time we have our code, we have two calls to
`print_vec`, one for a vector of strings and one for a vector of ints.
The call to `nm`...

Oh wait, I mentioned `nm` before, but let me tell you some more about it now!

A diversion about nm
--------------------

Here's what my manpage says:

    $ man nm

    NAME
           nm - display name list (symbol table)

    SYNOPSIS
           nm  [  -agnoprumxjlfPA  [  s segname sectname ]] [ - ] [ -t format ] [[
           -arch arch_flag ]...] [ file ... ]

    DESCRIPTION
           Nm displays the name list (symbol table) of each  object  file  in  the
           argument list.

Cool! You've never had to think about symbol tables before, so let's
talk about them.

When your compiler compiles something, you get an 'object file' out of
it. This is the binary that you run: `rustc fizzbuzz.rs` produces
`fizzbuzz`. This object file will contain a list of `symbols` and where
they exist in memory. This matters when we want to write two bits of
code that work together: If my library exposes a function called
`my_function`, and you want to use it, the compiler needs to know where
to find `my_function` in my library's code. The compiler 'mangles' the
names to fit its own scheme. This is called an "ABI", or application
binary interface. Have you ever seen this:

    /Users/Steve/.rvm/rubies/ruby-1.9.3-p286/lib/ruby/1.9.1

And wondered why that 1.9.1 is there? That's because Ruby 1.9.3 and Ruby
1.9.1 both share the same ABI, so gems that are linked against 1.9.1 can
also be used with 1.9.3. They use the same scheme to generate symbols.

Anyway, `nm` can show us this information. The first column is the
location in memory, the second is the (mangled) name:

    0000000100001bb8 S _rust_abi_version

That's a fun, recursive symbol ;) Anyway, we can examine what symbols
Rust exports to see some information about our executable, that's my
intention with `nm` in this case.

Back to our regularly scheduled investigation
---------------------------------------------

Here's the important part of the two outputs of nm:

    0000000000401500 t print_vec_2912::_85e5a3bc2d3e1a83::_00
    0000000000401ee0 t print_vec_2912::anon::expr_fn_2970
    0000000000404cd0 t print_vec_3218::_f1e1b4437dbb28a::_00
    0000000000405480 t print_vec_3218::anon::expr_fn_3252

and:

    00000000004012d0 t print_vec_2908::_85e5a3bc2d3e1a83::_00
    0000000000401cb0 t print_vec_2908::anon::expr_fn_2966

See how they both have `print_vec`? These are the functions we made. And
without even knowing what's happening, you can see the difference: in
the version of our code where we call `print_vec` on strings and ints,
we have two versions of the function, and on the version where we just
call it on ints, we have one version.

Neat! We get specialized versions, but only specialized for the types we
actually use. No generating code that's useless. This process is called
'monomorphization,' which basically means we take something that can work
with things of different types and change it (morph) into specialized
(mono) versions. To simplify, the compiler takes this code:

~~~ {.rust}
    fn print_vec<T: std::fmt::Show>(v: &[T]) {
        for i in v.iter() {
            println!("{}", i);
        }
    }

    fn main() {
        let vec = [1,2,3];

        print_vec(vec);

        let str_vec = [~"hey", ~"there", ~"yo"];

        print_vec(str_vec);
    }
~~~

And turns it into:

~~~ {.rust}
    fn print_vec_str(v: &[~str]) {
        for i in v.iter() {
            println!("{}", i);
        }
    }

    fn print_vec_int(v: &[int]) {
        for i in v.iter() {
            println!("{}", i);
        }
    }

    fn main() {
        let vec = [1,2,3];

        print_vec_int(vec);

        let str_vec = [~"hey", ~"there", ~"yo"];

        print_vec_str(str_vec);
    }
~~~

Complete with changing the calls at each call site to call the special
version of the function. We call this 'static dispatch,' as opposed to
the 'dynamic dispatch' that'd happen at runtime.

These are the kinds of optimizations that we get with static typing.
Neat! I will say that there are efforts to bring this kind of
optimization into dynamically typed languages as well, through analyzing
the call site. So, for example:

~~~ {.ruby}
    def foo(arg)
      puts arg
    end
~~~

If we call `foo` with a `String` `arg` a bunch of times in a row, the
interpreter will JIT compile a version of `foo` specialized for
`Strings`, and then replace the call site with something like:

~~~ {.ruby}
    if arg.kind_of? String
      __super_optimized_foo_string(arg)
    else
      foo(arg)
    end
~~~

This would give you the same benefit, without the human typing. Not just
that, but a sufficiently smart runtime would be able to actually
determine more complex situations that a person may not. And, maybe
after, say, 1000 calls with a String, just remove the check entirely.

Anyway.

Making our own Traits
---------------------

We want all of our monsters to implement `attack`. So let's make
`Monster` a Trait. The syntax looks like this:

~~~ {.rust}
    trait Monster {
        fn attack(&self);
    }
~~~

This says that the `Monster` trait guarantees we have one method
available on any type that implements the trait, `attack`. Here's how we
make one:

~~~ {.rust}
    trait Monster {
        fn attack(&self);
    }

    struct IndustrialRaverMonkey {
        strength: int
    }

    impl Monster for IndustrialRaverMonkey {
        fn attack(&self) {
            println!("The monkey attacks for {:d}.", self.strength)
        }
    }

    fn main() {
        let monkey = IndustrialRaverMonkey {strength:35};

        monkey.attack();
    }
~~~

Now we're cooking with gas! Remember our old implementation?:

~~~ {.rust}
    impl Monster {
        fn attack(&self) {
            match *self {
                ScubaArgentine(l, s, c, w) => println!("The monster attacks for {:d} damage.", w),
                IndustrialRaverMonkey(l, s, c, w) => println!("The monster attacks for {:d} damage.", w)
            }
        }
    }
~~~

Ugh. This is way better. No destructuring on types. We can write an
implementation for absolutely anything:

~~~ {.rust}
    trait Monster {
        fn attack(&self);
    }

    struct IndustrialRaverMonkey {
        strength: int
    }

    impl Monster for IndustrialRaverMonkey {
        fn attack(&self) {
            println!("The monkey attacks for {:d}.", self.strength)
        }
    }

    impl Monster for int {
        fn attack(&self) {
            println!("The int attacks for {:d}.", *self)
        }
    }

    fn main() {
        let monkey = IndustrialRaverMonkey {strength:35};
        monkey.attack();

        let i = 10;
        i.attack();
    }
~~~

Heh. Check it:

    $ rustc dwemthy.rs && ./dwemthy
    The monkey attacks for 35.
    The int attacks for 10.

Amusing.

Okay, exercise: Make six different monsters, and create a vector with
all of them in it. Then write a method that takes the vector, and prints
out all of the monsters and their stats.

I'll wait. It took me a little while to write this: this is the hardest
part of the book so far. Work through it; it'll be painful. Don't be
afraid to ask for help. I had to ask [the rust
IRC](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) for
help once while doing it. They're friendly, don't worry.

Done? Here's mine:

~~~ {.rust}
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
            println!("The monkey attacks for {:d}.", self.strength)
        }

        fn new() -> IndustrialRaverMonkey {
            IndustrialRaverMonkey {life: 46, strength: 35, charisma: 91, weapon: 2}
        }
    }

    impl Monster for DwarvenAngel {
        fn attack(&self) {
            println!("The angel attacks for {:d}.", self.strength)
        }
        fn new() -> DwarvenAngel {
            DwarvenAngel {life: 540, strength: 6, charisma: 144, weapon: 50}
        }
    }

    impl Monster for AssistantViceTentacleAndOmbudsman {
        fn attack(&self) {
            println!("The tentacle attacks for {:d}.", self.strength)
        }
        fn new() -> AssistantViceTentacleAndOmbudsman {
            AssistantViceTentacleAndOmbudsman {life: 320, strength: 6, charisma: 144, weapon: 50}
        }
    }

    impl Monster for TeethDeer {
        fn attack(&self) {
            println!("The deer attacks for {:d}.", self.strength)
        }
        fn new() -> TeethDeer {
            TeethDeer {life: 655, strength: 192, charisma: 19, weapon: 109}
        }
    }

    impl Monster for IntrepidDecomposedCyclist {
        fn attack(&self) {
            println!("The cyclist attacks for {:d}.", self.strength)
        }
        fn new() -> IntrepidDecomposedCyclist {
            IntrepidDecomposedCyclist {life: 901, strength: 560, charisma: 422, weapon: 105}
        }
    }

    impl Monster for Dragon {
        fn attack(&self) {
            println!("The dragon attacks for {:d}.", self.strength)
        }
        fn new() -> Dragon {
            Dragon {life: 1340, strength: 451, charisma: 1020, weapon: 939}
        }
    }

    fn monsters_attack(monsters: &[~Monster]) {
        for monster in monsters.iter() {
            monster.attack();
        }
    }

    fn main() {
        let monkey: ~IndustrialRaverMonkey               = ~Monster::new();
        let angel: ~DwarvenAngel                         = ~Monster::new();
        let tentacle: ~AssistantViceTentacleAndOmbudsman = ~Monster::new();
        let deer: ~TeethDeer                             = ~Monster::new();
        let cyclist: ~IntrepidDecomposedCyclist          = ~Monster::new();
        let dragon: ~Dragon                              = ~Monster::new();

        let dwemthys_vector: ~[~Monster] = ~[monkey as ~Monster, angel as ~Monster, tentacle as ~Monster, deer as ~Monster, cyclist as ~Monster, dragon as ~Monster];

        monsters_attack(dwemthys_vector);
    }
~~~

Congrats! You've mastered Traits. They're pretty awesome, right?
