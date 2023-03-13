# The Grid Geometry Manager

We'll take a bit of a break from talking about different widgets (what to put
onscreen) and focus instead on geometry management (where to put those widgets).
We introduced the general idea of geometry management in the "Tk Concepts"
chapter. Here, we focus on one specific geometry manager: `grid`.

As we've seen, grid lets you layout widgets in columns and rows. If you're
familiar with using HTML tables to do layout, you'll feel right at home here.
This chapter illustrates the various ways you can tweak grid to give you all the
control you need for your user interface.

Grid is one of several geometry managers available in Tk, but its mix of power,
flexibility, and ease of use make it the best choice for general use. Its
constraint model is a natural fit with today's layouts that rely on the
alignment of widgets. There are other geometry managers in Tk: `pack` is also
quite powerful, but harder to use and understand, while `place` gives you
complete control of positioning each element. Even widgets like paned windows,
notebooks, canvas, and text that we'll explore later can act as geometry
managers.

> It's worth noting that `grid` was first introduced to Tk in 1996, several
years after Tk became popular, and it took a while to catch on. Before that,
developers had always used `pack` to do constraint-based geometry management.
When `grid` came out, many developers kept using `pack`, and you'll still find
it used in many Tk programs and documentation. While there's nothing technically
wrong with `pack`, the algorithm's behavior is often hard to understand. More
importantly, because the order that widgets are packed is significant in
determining layout, modifying existing layouts can be more difficult. Aligning
widgets in different parts of the user interface is also much trickier.

> Grid has all the power of pack, produces nicer layouts (that align widgets
both horizontally and vertically), and is easier to learn and use. Because of
that, we think grid is the right choice for most developers most of the time.
Start your new programs using grid, and switch old ones to grid as you're making
changes to an existing user interface.

The [reference documentation](https://tcl.tk/man/tcl8.6/TkCmd/grid.htm) for grid
provides an exhaustive description of grid, its behaviors, and all options.
