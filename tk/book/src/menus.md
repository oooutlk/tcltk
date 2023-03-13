# Menus

This chapter describes how to handle menubars and popup menus in Tk. For a
polished application, these are areas you particularly want to pay attention to.
Menus need special care if you want your application to fit in with other
applications on your users' platform.

Speaking of which, the recommended way to figure out which platform you're
running on is:

```rust,no_run
tk.windowingsystem()?.to_string();  // returns "x11", "win32" or "aqua"
```

> This is more useful than examining global variables like `tcl_platform` or
`sys.platform`, and older checks that used these methods should be reviewed.
While in the olden days, there was a pretty good correlation between platform
and windowing system, it's less true today. For example, if your platform is
identified as Unix, that might mean Linux under X11, macOS under Aqua, or even
macOS under X11.
