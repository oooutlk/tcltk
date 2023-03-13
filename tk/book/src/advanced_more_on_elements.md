# Advanced: More on Elements

While that's about as far as we're going to go on styles and themes in this
tutorial, for curious users and those who want to delve further into creating
new themes, we can provide a few more interesting tidbits about elements.

Because elements are the building blocks of styles and themes, it begs the
question of "where do elements come from?" Practically speaking, we can say that
elements are normally created in C code and conform to a particular API that the
theming engine understands.

At the very lowest level, elements come from something called an element
factory. At present, there is a default one, which most themes use, and uses Tk
drawing routines to create elements. A second allows you to create elements from
images and is accessible at the script level using the `Tk::element_create`
method (from Tcl). Any image format supported by Tk is available, including
scalable image formats like SVG, if you have the right extension. Finally, there
is a third, Windows-specific engine using the underlying "Visual Styles"
platform API.

If a theme uses elements created via a platform's native widgets, the calls to
use those native widgets will normally appear within that theme's element
specification code. Of course, themes whose elements depend on native widgets or
API calls can only run on the platforms that support them.

Themes will then take a set of elements and use those to assemble the styles
that are actually used by the widgets. And given the whole idea of themes is
that several styles can share the same appearance, it's not surprising that
different styles share the same elements.

So while the `TButton` style includes a `Button.padding` element, and the
`TEntry` style includes an `Entry.padding` element, underneath, these padding
elements are more than likely one and the same. They may appear differently, but
that's because of different configuration options, which, as we recall, are
stored in the style that uses the element.

It's also probably not surprising to find out that a theme can provide a set of
common options that are used as defaults for each style if the style doesn't
specify them otherwise. This means that if pretty much everything in an entire
theme has a green background, the theme doesn't need to explicitly say this for
each style. This uses a root style named `"."`. If `Fun.TButton` can inherit
from `TButton`, why can't `TButton` inherit from `"."`?

Finally, it's worth having a look at how existing themes are defined, both at
the C code level in Tk's C library and via the Tk scripts found in Tk's
"library/ttk" directory or in third-party themes. Search for
`Ttk_RegisterElementSpec` in Tk's C library to see how elements are specified.
