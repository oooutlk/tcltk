# Nested Layouts

As your user interface gets more complicated, the grid that you're using to
organize all your widgets can get increasingly complicated. This can make
changing and maintaining your program very difficult.

Luckily, you don't have to manage your entire user interface with a single grid.
If you have one area of your user interface that is fairly independent of
others, create a new frame to hold that area and grid the widgets in area within
that frame. For example, if you were building a graphics editor with multiple
palettes, toolbars, etc., each one of those areas might be a candidate for
putting in its own frame.

In theory, these frames, each with its own grid, can be nested arbitrarily deep,
though, in practice, this usually doesn't go beyond a few levels. This can be a
big help in modularizing your program. If, for example, you have a palette of
drawing tools, you can create the whole thing in a separate function or class.
It would be responsible for creating all the component widgets, gridding them
together, setting up event bindings, etc. The details of how things work inside
that palette can be contained in that one piece of code. From the point of view
of your main program, all it needs to know about is the single frame widget
containing your palette.

Our examples have shown just a hint of this, where a content frame was gridded
into the main window, and then all the other widgets gridded into the content
frame.

As your own programs grow larger, you'll likely run into situations where making
a change in the layout of one part of your interface requires code changes to
the layout of another part. That may be a clue to reconsider how you're using
`grid` and if splitting out components into separate frames would help.
