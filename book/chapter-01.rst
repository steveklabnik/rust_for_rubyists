{% import 'macros/ork.jinja' as ork with context %}

Jazz Pentatonics Example
========================

There are two basic pentatonics you can play for Jazz.  One is based on the
Major Scale, the other is based on the Mixolydian mode.  If I write
these out in the key of C they are:

.. image:: examples/jazz_pents.{{ ork.image_ext }}

These two pentatonics, when shifted make up all the main arpeggios with common
additional notes, but without any of the avoid notes like the 4th and the 7th.

The Scales
----------

To print out the notes in these pentatonics for all keys, I use this code:

{{ ork.code("examples/scale_notes.py|idio") }}

With just these two versions of the scale I can then play over chords
in various ways.

The Chords
----------

To analyze all the most useful permutations I have a script that goes through
every chord I need and then tries all the pentatonics to see if any match the
same notes in the chords.

First I setup a bunch of sets for the notes that are in the kinds of
chords I like to play:

{{ ork.codes("examples/chord_analysis.py|idio", "chords") }}

Notice how I'm importing the ``scale_notes`` module that I just showed
you.

I then have an ``analyze_scales_chords`` function which goes through
all the permutations of chords and the notes from the ``scale_notes``
module to find all the possible matches.

{{ ork.codes("examples/chord_analysis.py|idio", "analyze") }}

A match is simply any pentatonic that has the same notes as might be
found in that kind of chord.

Once I have that I then need to reduce it to something I can print out:

{{ ork.codes("examples/chord_analysis.py|idio", "reduce") }}

And finally I just run it twice.  Once to get all the fits for the
Dominant Scales (Jazz Pentatonics), and once again for the Major Scales
(Blues Pentatonics):

{{ ork.codes("examples/chord_analysis.py|idio", "final") }}

The Results
-----------

Here's the results::

    {{ d["examples/chord_analysis.py|py"].as_text()|indent(4, false) }}

Here's some examples of these scales for different chords using ABC notation::

    {{ d["examples/jazz_pents_example.abc|ss"]|indent(4, false) }}

And finally what that looks like rendered into SVG:

.. image:: examples/jazz_pents_example.{{ ork.image_ext }}


