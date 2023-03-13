# Creating and Destroying Windows

We've seen that all Tk programs start out with a root toplevel window, and then
widgets are created as children of that root window. Creating new toplevel
windows works almost exactly the same as creating new widgets.

Toplevel windows are created using the `toplevel` method:

```rust,no_run
let window = parent.add_toplevel(())?;
```

Note: Toplevels are part of the classic Tk widgets, not the themed widgets.

Unlike regular widgets, we don't have to `grid` a toplevel for it to appear
onscreen. Once we've created a new toplevel, we can create other widgets as
children of that toplevel, and `grid` them inside the toplevel. The new toplevel
behaves exactly like the automatically created root window.

To destroy a window, use the `destroy` method:

```rust,no_run
tk.destroy( window )?;
```

Note that you can use `destroy` on any widget, not just a toplevel window. When
you destroy a window, all windows (widgets) that are children of that window are
also destroyed. Be careful! If you destroy the root window (that all other
widgets are descended from), that will terminate your application.

> In a typical document-oriented application, we want to be able to close any
windows while leaving the others open. In that case, we may want to create a new
toplevel for every window, and not put anything directly inside the root window
at all. While we can't just destroy the root window, we can remove it entirely
from the screen using its `withdraw` method, which we'll see shortly.
