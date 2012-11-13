{% import 'macros/ork.jinja' as ork with context %}

Orkestrix Music Publishing System
=================================


Final Experiment
----------------

This is the last experiment that I'm doing to explore the features
I need for Orkestrix_.  The next releases will be a combination
of working software and documentation with examples on using it.

.. contents::

Introduction
------------

This is a small sample document to document how I'm building a 
way to write HTML (or PDF, etc.) that easily has music notation
in it.  Yes, I know about LilyPond and it's too complex for what
I want.  I want something that I can hand to musicians and they
can get writing pretty quickly.  These days everyone knows some
kind of WikiText markup language, so I'm aiming for that style
of document workflow.

I want to try combining three things:

1. reStructuredText_ For writing the prose.
2. ABC_ For writing the music.
3. Dexy_ For building it all and because it's awesome.

From these I can output nearly everything I need very easily.  Here's how
I'm doing it so far.

Getting Dexy
------------

The first thing is to setup dexy and get your docs.yaml going.  Right
now you have to install dexy from the git repository, so here's how you
do that::

    git clone https://github.com/ananelson/dexy.git
    cd dexy
    sudo python setup.py install

That should download all your dependencies and give you the dexy command.

Once you have that installed, you simply create a directory and tell dexy
to set it up::

    mkdir music-sample
    cd music-sample
    dexy setup

Last thing you do is make a ``docs.yaml`` file that sets up some build
parameters for building rST and ABC documents:


.. @export "abc"

{{ ork.code('docs.yaml|pyg') }}

.. @end

Here is how to include docs.yaml:

{{ ork.codes('index.rst|idio', 'abc') }}

This seems large but here's the breakdown of what it does:

1. I have a songs section that lists out how to process ABC_ files.
   Under that I have filters setup for processing the ABC_ as SVG
   or as EPS files.  Dexy uses a filter specification that's 
   ``file|filter|filter`` then you can add options for each filter 
   under it.  The ``file`` part can also just a ``.ext`` file extension
   to mean "all the files ending in this".
2. I do the same thing for ``assets`` and ``code``.
3. I create a target ``.rst|jinja|rst2html`` which depends on the
   ``songs``, ``assets``, and ``code`` targets, then sets options for
   the ``rst2html`` plugin so that I can alter how rST_ outputs my
   stuff.
4. Finally I do the same thing but for ``.rst|jinja|rst2latex|xetex``
   which will run it through xetex instead of doing HTML.

Yes, this means we can run our rST_ and ABC_ files through Jinja_ first, which
gives us fun templating features.  This is of course all for free because Dexy
has Jinja built in.

.. note::

    In the current Phase 6 version the docs.yaml file is more complex so that
    I can style the rST_ output for HTML and LaTeX, but the above is the
    basics of it.  Look in the github_ repo to see how it's currently
    implemented.

Writing The ABC
---------------

Once you have this you need some ABC to work with, and the ABC_ site
hace plenty to play with.  Here's one simple one I grabbed::

{{ d['sample.abc|jinja|ss'] }}

You can't see the Dexy command I used, but just view the source_ of this
document and you can check what I did.

How Dexy Finds Stuff
--------------------

Dexy basically processes all of your documents and source materials,
runs them through the filtes you requested in the dependencies you wanted.

1. First dexy makes a big dictionary with key=value pairs for each result.
2. You refer to the documents you want to include simply by using their 
   ``file + filter`` name, so in the above I just use ``sample.abc|jinja|ss``.
   If I want another ``.abc`` file I use ``other.abc|jinja|ss``.
3.  These keys are just kept in a ``d[]`` variable for jinja, and that 
    means you just do ``d['sample.abc|jinja|ss']``.
4. That means you put that in a jinja tag and that's it. Dexy figures it out
   and injects what you need.


Writing The rST
---------------

Once you have that written you write the rST_ with what you want to say, and
you include the Dexy commands you want to get your files in.  The magic of
dexy is that you don't just include docs, you include docs piped through
filters.

To see how to write an rST, take a look at the source_ to this document
as an example.

Including The ABC As SVG or EPS
-------------------------------

Finally, to get the ``sample.abc`` file into the ``intro.rst`` file
outputs you do:

``.. image:: sample.{{ ork.image_ext }}``

This uses a variable I set in the ``dexy.yaml`` that lets me know what
the extension is for the file in that particular run.  If dexy is making the
``.pdf`` using the ``.rst|jinja|rst2latex|latex`` filter then I get ``eps``.
If it's doing the other ``.rst|jinja|rst2html`` filter then I get ``svg``.
That is matched up with the ``abcm2ps`` command Dexy runs for the ``AbcFilter``.

I could also use all of Jinja to alter the output or rST_ however I want.

The final result then looks like:

.. image:: sample.{{ ork.image_ext }}

This now lets me produce HTML or PDFs from an rST_ document, but tailor the
generated resources based on the target output.

Running Dexy
------------

Last step is you just run dexy::

    dexy

If you did everything right then you should get a document that looks like
what I've got here.

The PDF Version
---------------

You can take a look at the PDF_ version of this same document that is produced
with the above single command from the one source set.


Why This Is Fun
---------------

Here's some key points to understand about what I just did:

1. That's straight up SVG, which means I can style it with CSS and it works in most browsers.
2. I can also produce an EPS from this, then use rst2latex, and put this same music in a PDF output.
3. If you look at the source_ it's 1 line to do that and it matches the source I mention above. No more source/output skew.
4. ABC_ is like markdown for music notation. rST_ is a nice strict multi-output format for text. Dexy_ binds them together
   and gives me templating and document carving awesomeness for free.
5. Dexy doesn't get in my way when I'm craft a document workflow.  Other tools impose their stupid ideas about how you should
   write your docs, while dexy just gives you the framework that has 90% of the crap document workflows have that you 
   kind of don't care about.  And, if you do care about it you can write your own filters and plugins to change it.
6. If you try to write about music you'll realize why the above is great stuff.  If you write about code you should see
   why this is also great stuff.

Sample Of Including Colorized Code
----------------------------------

One additional thing I'll want to do, since I'm a programmer, is include code
that I may write about music.  Here's a simple example of getting that included
with Pygments_ coloring:

{{ ork.code('test.py|pyg') }}

You include code using the ``ork`` macros I'm making.  Here's a simple sample
that does both the import and loading of some code:

.. code::

    {% raw %}
    {% import 'macros/ork.jinja' as ork with context %}
    {{ ork.code('test.py|pyg') }}
    {% endraw %}

Finally here's a bigger sample that is the ``macros/ork.jinja`` file itself:

{{ ork.code('macros/ork.jinja|pyg') }}

That is a lot of Junk if you don't know how to code, but this is a nice demo
of including code in your documents directly with color.

Conclusion
----------

This is the last experiment phase for seeing if Orkestrix_ will work
to produce the kind of documents I like.  I'm now going to start formalizing
how it's structured and how to use it.  This site will eventually change
to simply document how to use Orkestrix_ and also have some sample
documents demonstrating it.

.. _reStructuredText: http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html
.. _rST: http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html
.. _ABC: http://abcnotation.com/
.. _Dexy: http://dexy.it/
.. _YAML: http://www.yaml.org/
.. _Jinja: http://jinja.pocoo.org/
.. _source: http://orkestrix.org/index.rst
.. _PDF: http://orkestrix.org/index.pdf
.. _zedshaw: http://twitter.com/zedshaw
.. _Orkestrix: http://orkestrix.org/
.. _github: http://github.org/zedshaw/orkestrix
.. _Pygments: http://pygments.org/
