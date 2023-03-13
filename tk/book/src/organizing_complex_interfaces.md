# Organizing Complex Interfaces

If you have a complex user interface, you'll need to find ways to organize it
that don't overwhelm your users. There are several different approaches to doing
this. Both general-purpose and platform-specific human interface guidelines are
good resources when designing your user interface.

When we talk about complexity in this chapter, we don't mean the underlying
technical complexity of how the program is implemented. Instead, we mean how
it's presented to users. A user interface can be pulled together from many
different modules, built from hundreds of widgets combined in a deeply nested
hierarchy, but that doesn't mean users need to perceive it as complex.

## Multiple windows

One benefit of using multiple windows in an application can be to simplify the
user interface. Done well, it can require users to focus only on the contents of
one window at a time to complete a task. Forcing them to focus on or switch
between several windows can also have the opposite effect. Similarly, showing
only the widgets relevant to the current task (i.e., via `grid`) can help
simplify the user interface.

## White space

If you do need to display a large number of widgets onscreen at the same time,
think about how to organize them visually. We've seen how `grid` makes it easy
to align widgets with each other. White space is another useful aid. Place
related widgets close to each other (possibly with an explanatory label
immediately above) and separate them from other widgets by white space. This
helps users organize the user interface in their own minds.
