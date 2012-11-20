The Orkestrix Manual
********************

This is a simple demo of doing a book that has three output formats:

1. A single PDF_ with all chapters in it.
2. A single HTML_ page with all chapters in it.
3. This page with each chapter separate.

Current Structure Of A Book
---------------------------

If you grab the github_ repo you'll see that there's two basic
levels of doing a "website that has a book":

1. The root of the project which is just the website, matching up with orkestrix.org.  This has all the javascript and stylesheets and its own template.
2. A ``book/`` directory with the book in it.

In the ``book/`` directory you have:

* ``book.rst`` -- This encapsulates the full PDF_ or HTML_ of all chapters and you manually import each chapter you need in the order you want.
* ``index.rst`` -- That's this page, and it lets you do links to each chapter as individual files.
* ``chapter-*.rst`` -- Each chapter that you're writing.  These actually could be called anything with .rst, but to keep it sane I went with ``chapter-*.rst``.  This way it doesn't conflict with ``book.rst`` or ``index.rst``.
* ``template.html`` -- A separate template so you can style the book differently than the rest of the site.  Usually books have different navigation and need to be wider so they need their own template.

What ``book.rst`` Contains
--------------------------

To setup the ``book.rst`` you just use a simple Jinja include of the
chapters you want in the order you want using::

    {% raw %}
    {% include "book/chapter-01.rst" %}
    {% endraw %}

These can be called anything you want, so it could be ``chapter-the-cool-stuff.rst``, and if you don't like starting it with ``chapter`` you can just change the ``dexy.yaml`` to use a different name.

With that the rST system will produce the table of contents for you and you're done.

Table Of Contents
-----------------

* `Chapter 1`_ A Sample Chapter

.. _Chapter 1: chapter-01.html
.. _PDF: book.pdf
.. _HTML: book.html
.. _github: http://github.org/zedshaw/orkestrix

