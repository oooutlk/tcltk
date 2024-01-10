# Window Behavior and Styles

There are lots of things about how windows behave and how they look that can be
changed.

## Window Title

To examine or change the title of the window:

```rust,no_run
let old_title = window.wm_title()?;
window.wm_title( "New title" )?;
```

> The "wm" here stands for "window manager" which is an X11 term used for a
program that manages all the windows onscreen, including their title bars,
frames, and so on. What we're effectively doing is asking the window manager to
change the title of this particular window for us. The same terminology has been
carried over to Tk running on macOS and Windows.

### Size and Location

Here is an example of changing the size and position. It places the window
towards the top righthand corner of the screen:

```rust,no_run
window.set_wm_geometry( TkGeometry{ w: 300, h: 200, x: -5, y: 40 })?;
```

You can retrieve the current geometry using `wm_geometry` method. However, if
you try it immediately after changing the geometry, you'll find it doesn't
match. Remember that all drawing effectively occurs in the background, in
response to idle times via the event loop. Until that drawing occurs, the
internal geometry of the window won't be updated. If you do want to force things
to update immediately, you can.

```rust,no_run
tk.update_idletasks()?;
println!( "{}", window.wm_geometry()? );
```

We've seen that the window defaults to the size requested by the widgets that
are gridded into it. If we're creating and adding new widgets interactively in
the interpreter, or if our program adds new widgets in response to other events,
the window size adjusts. This behavior continues until either we explicitly
provide the window's geometry as above or a user resizes the window. At that
point, even if we add more widgets, the window won't change size. You'll want to
be sure you're using all of `grid`'s features (e.g., `sticky`, `weight`) to make
everything fit nicely.

### Resizing Behavior

By default, toplevel windows, including the root window, can be resized by
users. However, sometimes you may want to prevent users from resizing the
window. You can do this via the `resizable` method. It's first parameter
controls whether users can change the width, and the second if they can change
the height. So to disable all resizing:

```rust,no_run
window.set_wm_resizable( false, false )?;
```

If a window is resizable, you can specify a minimum and/or maximum size that
you'd like the window's size constrained to (again, parameters are width and
height):

```rust,no_run
window.set_wm_minsize( 200, 100 )?;
window.set_wm_maxsize( 500, 500 )?;
```

You saw earlier how to obtain the current size of the window via its geometry.
Wondering how large it would be if you didn't specify its geometry, or a user
didn't resize it? You can retrieve the window's requested size, i.e., how much
space it requests from the geometry manager. Like with drawing, geometry
calculations are only done at idle time in the event loop, so you won't get a
useful response until the widget has appeared onscreen.

```rust,no_run
window.winfo_reqwidth()?; // or reqheight
```

> You can use the `winfo_reqwidth` and `winfo_reqheight` methods on any widget,
not just toplevel windows. There are other `winfo_*` methods you can call on any
widget, such as width and height, to get the actual (not requested) width and
height. For more, see the `winfo` mod.

### Intercepting the Close Button

Most windows have a close button in their title bar. By default, Tk will destroy
the window if users click on that button. You can, however, provide a callback
that will be run instead. A common use is to prompt the user to save an open
file if modifications have been made.

```rust,no_run
unsafe { window.set_wm_protocol( "WM_DELETE_WINDOW", callback )?; }
```

> The somewhat obscurely-named `WM_DELETE_PROTOCOL` originated with X11 window
manager protocols.

### Transparency

Windows can be made partially transparent by specifying an alpha channel,
ranging from `0.0` (fully transparent) to `1.0` (fully opqaque).

```rust,no_run
window.set_wm_attributes( -alpha(0.5) )?;
```

On macOS, you can additionally specify a `-transparent` attribute (using the
same mechanism as with `-alpha`), which allows you to make the background of the
window transparent, and remove the window's show. You should also set the
`background` configuration option for the window and any frames to the color
`ssytemTransparent`.

### Full Screen

You can make a window expand to take up the full screen:

```rust,no_run
window.set_wm_attributes( -fullscreen(true) )?;
```

### Iconifying and Withdrawing

On most systems, you can temporarily remove the window from the screen by
iconifying it. In Tk, whether or not a window is iconified is referred to as the
window's state. The possible states for a window include `normal` and `iconic`
(for an iconified window), and several others: `withdrawn`, `icon` or `zoomed`.

You can query or set the current window state directly. There are also methods
`iconify`, `deiconify`, and `withdraw`, which are shortcuts for setting the
`iconic`, `normal`, and `withdrawn` states, respectively.

```rust,no_run
let the_state = window.wm_state()?;
window.set_wm_state( TkState::Normal )?;
window.wm_iconify()?;
window.wm_deiconify()?;
window.wm_withdraw()?;
```

### Stacking Order

Stacking order refers to the order that windows are "placed" on the screen, from
bottom to top. When the positions of two windows overlap each other, the one
closer to the top of the stacking order will obscure or overlap the one lower in
the stacking order.

You can ensure that a window is always at the top of the stacking order (or at
least above all others where this attribute isn't set):

```rust,no_run
window.set_wm_attributes( -topmost(true) )?;
```

You can find the current stacking order, listed from lowest to highest:

```rust,no_run
window
    .wm_stackorder()?
    .iter()
    .for_each( |widget| println!( "stackorder: {}", widget.path() ));
```

You can also just check if one window is above or below another:

```rust,no_run
if window.wm_stackorder_isabove( &other ) {}
if window.wm_stackorder_isbelow( &other ) {}
```

You can also raise or lower windows, either to the very top (bottom) of the
stacking order, or just above (below) a designated window:

```rust,no_run
window.raise()?;
window.raise_above( &other )?;
window.lower()?;
window.lower_below( &other )?;
```

Why do you need to pass a window to get the stacking order? Stacking order
applies not only for toplevel windows, but for any sibling widgets (those with
the same parent). If you have several widgets gridded together but overlapping,
you can raise and lower them relative to each other:

```rust,no_run
let little = root.add_ttk_label( "little" -text("Little") )?
    .grid( -column(0) -row(0) )?; 
root.add_ttk_label( "bigger" -text("Much Bigger Label") )?
    .grid( -column(0) -row(0) )?;
tk.after( 2000, (tkbind!( tk,
    || -> TkResult<()> { Ok( little.raise()? )}),))?;
```

## Screen Information

We've previously used the `winfo` command to find out information about specific
widgets. It can also provide information about the entire display or screen. As
usual, see the `winfo` command reference for full details.

For example, you can determine the screen's color depth (how many bits per
pixel) and color model (usually `truecolor` on modern displays), it's pixel
density, and resolution.

```rust,no_run
println!( "color depth={} ({})", root.winfo_screendepth()?, root.winfo_screenvisual()? );
println!( "pixels per inch={}", root.winfo_pixels( TkDistance::Inches(1.0) )? );
println!( "width={} height={}", root.winfo_screenwidth()?, root.winfo_screenheight()? );
```

### Multiple Monitors

While normally you shouldn't have to pay attention to it, if you do have
multiple monitors on your system and want to customize things a bit, there are
some tools in Tk to help.

First, there are two ways that multiple monitors can be represented. The first
is with logically separate displays. This is often the case on X11 systems,
though it can be changed, e.g., using the `xrandr` system utility. A downside of
this model is that once a window is created on a screen, it can't be moved to a
different one. You can determine the screen that a Tk window is running on,
which looks something like `:0.0` (an X11-formatted display name).

```rust,no_run
root.winfo_screen()?;
```

When first creating a `toplevel` you can specify the screen it should be created
on using the `screen` configuration option.

> Different monitors may have different resolutions, color depths, etc. You'll
notice that all the screen information calls we just covered are methods invoked
on a specific widget. They will return information about whatever screen that
window is located on.

Alternatively, multiple monitors can also be represented as one big virtual
display, which is the case on macOS and Windows. When you ask for information
about the screen, Tk will return information on the primary monitor. For
example, if you have two Full HD monitors side-by-side, the screen resolution
will be reported as 1920 x 1080, not 3840 x 1080. This is probably a good thing;
it means that if we're positioning or sizing windows, we don't need to worry
about multiple monitors, and everything will just show up correctly on the
primary monitor.

What if a user moves a window from the primary monitor to a different one? If
you ask for its position, it will be relative to the primary monitor. So in our
side-by-side FHD monitor setup, if you call the `winfo_x` method on a window
positioned near the left edge of a monitor, it might return `100` (if it's on
the primary monitor), `-1820` (if it's on a monitor to the left of the primary
monitor), or `2020` (if it's on a monitor to the right of the primary monitor).
You can still use the `geometry` method we saw a bit earlier to position the
window on a different monitor, even though the geometry specification may look a
bit odd, e.g., `x:-1820, y:100`.

You can find out approximately how large the entire display is, spanning
multiple monitors. To do so, check a toplevel widget's maximum size, i.e., how
large the user can resize it (you can't do this after you've already changed it,
of course). This may be a bit smaller than the full size of the display. For
example, on macOS, it will be reduced by the size of the menubar at the top of
the screen.

```rust,no_run
root.wm_maxsize()?;
```

## Run Example

`cargo run --example window_behavior_and_styles`
