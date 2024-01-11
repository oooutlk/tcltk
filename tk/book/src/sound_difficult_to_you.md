# Sound Difficult to you?

You now know that styles consist of elements, each with various options,
composed together in a layout. You can change options on styles to make all
widgets using the style appear differently. Any widgets using that style take on
the appearance that the style defines. Themes collect an entire set of related
styles, making it easy to change the appearance of your entire user interface.

So what makes styles and themes so difficult in practice? Three things. First:

    You can only modify options for a style, not element options (except sometimes).

We talked earlier about identifying the elements used in the style by examining
its layout and identifying what options were available for each element. But
when we went to make changes to a style, we seemed to be configuring an option
for the style without specifying an individual element. What's going on?

Again, using our button example, we had an element `Button.label`, which, among
other things, had a `font` configuration option. What happens is that when that
`Button.label` element is drawn, it looks at the `font` configuration option set
on the style to determine what font to draw itself in.

> To understand why, you need to know that when a style includes an element as
a piece of it, that element does not maintain any (element-specific) storage. In
particular, it does not store any configuration options itself. When it needs to
retrieve options, it does so via the containing style, which is passed to the
element. Individual elements, therefore, are "flyweight" objects in GoF pattern
parlance.

Similarly, any other elements will look up their configuration options from
options set on the style. What if two elements use the same configuration option
(like a background color)? Because there is only one background configuration
option (stored in the style), both elements will use the same background color.
You can't have one element use one background color and the other use a
different background color.

> Except when you can. There are a few nasty, widget-specific things called
*sublayouts* in the current implementation, which let you sometimes modify just
a single element, via configuring an option like `TButton.Label` (rather than
just `TButton`, the name of the style).

> Some styles also provide additional configuration options that let you specify
what element the option affects. For example, the `TCheckbutton` style provides
a `background` option for the main part of the widget and an
`indicatorbackground` option for the box that shows whether it is checked.

> Are the cases where you can do this documented? Is there some way to
introspect to determine when you can do this? The answer to both questions is
"sometimes" (believe it or not, this is an improvement; the answer to both used
to be a clear "no"). You can sometimes find *some* of the style's options by
calling the style's `configure` method without providing any new configuration
options. The reference manual pages for each themed widget now generally include
a *styling options* section that lists options that *may* be available to
change.

> This is one area of the themed widget API that continues to evolve over time.

The second difficulty is also related to modifying style options:

    Available options don't necessarily have an effect, and it's not an error to
    modify a bogus option.

You'll sometimes try to change an option that is supposed to exist according to
element options, but it will have no effect. For example, you can't modify the
background color of a button in the `aqua` theme used by macOS. While there are
valid reasons for these cases, it's not easy to discover them, which can make
experimenting frustrating at times.

Perhaps more frustrating when you're experimenting is that specifying an
`incorrect` style name or option name does not generate an error. When doing a
`configure` or `lookup` you can provide an entirely arbitrary name for a style
or an option. So if you're bored with the `background` and `font` options, feel
free to configure a `dowhatimean` option. It may not do anything, but it's not
an error. Again, it may make it hard to know what you should be modifying and
what you shouldn't.

> This is one of the downsides of having a very lightweight and dynamic system.
You can create new styles by providing their name when configuring style options
without explicitly creating a style object. At the same time, this does open
itself to errors. It's also not possible to find out what styles currently exist
or are used. And remember that style options are really just a front end for
element options, and the elements in a style can change at any time. It's not
obvious that options should be restricted to those referred to by current
elements alone, which may themselves not all be introspectable.

Finally, here is the last thing that makes styles and themes so difficult:

    The elements available, the names of those elements, which options are
    available or affect each of those elements, and which are used for a
    particular widget can be different in every theme.

So? Remember, the default theme for each platform (Windows, macOS, and Linux) is
different (which is a good thing). Some implications of this:

1. If you want to define a new type of widget (or a variation of an existing
widget) for your application, you'll need to do it separately and differently
for each theme your application uses (i.e., at least three for a cross-platform
application).

2. As the elements and options available may differ for each theme/platform, you
may need a quite different customization approach for each theme/platform.

3. The elements, names, and element options available with each theme are not
typically documented (outside of reading the theme definition files themselves)
but are generally identified via theme introspection (which we'll see soon).
Because all themes aren't available on all platforms (e.g., `aqua` is only
available on macOS), you'll need ready access to every platform and theme you
need to run on.

Consider trying to customize a button. You know it uses the `TButton` style. But
that style is implemented using a different theme on each platform. If you
examine the layout of that style in each theme, you'll discover each uses
different elements arranged differently. If you try to find the advertised
options available for each element, you see those are different too. And of
course, even if an option is nominally available, it may not have an effect).

The bottom line is that in classic Tk, where you could modify any of a large set
of attributes for an individual widget, you'd be able to do something on one
platform, and it would sorta-kinda work (but probably need tweaking) on others.
In themed Tk, the easy option just isn't there, and you're pretty much forced to
do it the right way if you want your application to work with multiple themes/
platforms. It's more work upfront.
