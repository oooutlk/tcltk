# Geometry Management

If you've been running code interactively, you've probably noticed that just by
creating widgets, they didn't appear onscreen. Placing widgets onscreen, and
precisely where they are placed, is a separate step called geometry management.

In our example, positioning each widget was accomplished by the grid command. We
specified the column and row we wanted each widget to go in, how things were to
be aligned within the grid, etc. Grid is an example of a geometry manager (of
which there are several in Tk, grid being the most useful). For now, we'll look
at geometry management in general; we'll talk about grid in a later chapter.

A geometry manager's job is to figure out exactly where those widgets are going
to be put. This turns out to be a complex optimization problem, and a good
geometry manager relies on quite sophisticated algorithms. A good geometry
manager provides the flexibility, power, and ease of use that makes programmers
happy. It also makes it easy to create good looking user interface layouts
without needing to jump through hoops. Tk's grid is, without a doubt, one of the
absolute best. A poor geometry manager... well, all the Java programmers who
have suffered through "GridBagLayout" please raise their hands.

> We'll go into more detail in a later chapter, but grid was introduced several
years after Tk became popular. Before that, an older geometry manager named pack
was most commonly used. It's very powerful, but is much harder to use, and makes
it extremely difficult to create layouts that look appealing today.
Unfortunately, much of the example Tk code and documentation out there uses pack
instead of grid (a good clue to how current it is). The widespread use of pack
is one major reason that so many Tk user interfaces look terrible. Start new
code with grid, and upgrade old code when you can.

## The Problem

The problem for a geometry manager is to take all the different widgets the
program creates, plus the program's instructions for where in the window each
should go (explicitly, or more often, relative to other widgets), and then
actually position them in the window.

In doing so, the geometry manager has to balance multiple constraints. Consider
these situations:

- The widgets may have a natural size, e.g., the natural width of a label would
depend on the text it displays and the font used to display it. What if the
application window containing all these different widgets isn't big enough to
accommodate them? The geometry manager must decide which widgets to shrink to
fit, by how much, etc.

- If the application window is bigger than the natural size of all the widgets,
how is the extra space used? Is extra space placed between each widget, and if
so, how is that space distributed? Is it used to make certain widgets larger
than they normally want to be, such as a text entry growing to fill a wider
window? Which widgets should grow?

- If the application window is resized, how does the size and position of each
widgets inside it change? Will certain areas (e.g., a text entry area) expand or
shrink while other parts stay the same size, or is the area distributed
differently? Do certain widgets have a minimum size that you want to avoid going
below? A maximum size? Does the window itself have a minimum or maximum size?

- How can widgets in different parts of the user interface be aligned with each
other? How much space should be left between them? This is needed to present a
clean layout and comply with platform-specific user interface guidelines.

- For a complex user interface, which may have many frames nested in other
frames nested in the window (etc.), how can all the above be accomplished,
trading off the conflicting demands of different parts of the entire user
interface?

# How it Works

Geometry management in Tk relies on the concept of master and slave widgets. A
master is a widget, typically a toplevel application window or a frame, which
contains other widgets, called slaves. You can think of a geometry manager
taking control of the master widget and deciding how all the slave widgets will
be displayed within.

> The computing community has embraced the more general societal trend towards
more diversity, sensitivity, and awareness about the impacts of language.
Recognizing this, the Tk core will slowly be adopting a more inclusive set of
terminology. For example, where it makes sense, "parent" and "child" will be
preferred over "master" and "slave." To preserve backward compatibility, the
current terminology will not be disappearing. This is something to be aware of
for the future. For more details, see [TIP #581](https://tip.tcl.tk/581).

Your program tells the geometry manager what slaves to manage within the master,
i.e., via calling `grid`. Your program also provides hints as to how it would
like each slave to be displayed, e.g., via the `column` and `row` options. You
can also provide other things to the geometry manager. For example, we used
`columnconfigure` and `rowconfigure` to indicate the columns and rows we'd like
to expand if there is extra space available in the window. It's worth noting
that all these parameters and hints are specific to `grid`; other geometry
managers would use different ones.

The geometry manager takes all the information about the slaves in the master,
as well as information about how large the master is. It then asks each slave
widget for its natural size, i.e., how large it would ideally like to be
displayed. The geometry manager's internal algorithm calculates the area each
slave will be allocated (if any!). The slave is then responsible for rendering
itself within that particular rectangle. And of course, any time the size of the
master changes (e.g., because the toplevel window was resized), the natural size
of a slave changes (e.g., because we've changed the text in a label), or any of
the geometry manager parameters change (e.g., like `row`, `column`, or `sticky`)
we repeat the whole thing.

This all works recursively as well. In our example, we had a content frame
inside the toplevel application window, and then several other widgets inside
the content frame. We, therefore, had to manage the geometry for two different
masters. At the outer level, the toplevel window was the master, and the content
frame was its slave. At the inner level, the content frame was the master, with
each of the other widgets being slaves. Notice that the same widget, e.g., the
content frame, can be both a master and a slave! As we saw previously, this
widget hierarchy can be nested much more deeply.

> While each master can be managed by only one geometry manager (e.g. `grid`),
different masters can have different geometry managers. While `grid` is the
right choice most of the time, others may make sense for a particular layout
used in one part of your user interface. Other Tk geometry managers include
`pack`, which we've mentioned, and `place`, which leaves all layout decisions
entirely up to you. Some complex widgets like `canvas` and `text` let you embed
other widgets, making them de facto geometry managers.

> Finally, we've been making the assumption that slave widgets are the immediate
children of their master in the widget hierarchy. While this is usually the
case, and mostly there's no good reason to do it any other way, it's also
possible (with some restrictions) to get around this.
