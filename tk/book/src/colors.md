# Colors

As with fonts, there are various ways to specify colors. Full details can be
found in the
[colors command reference](https://tcl.tk/man/tcl8.6/TkCmd/colors.htm).

In general, Tk widgets default to the right colors for most situations. If you'd
like to change colors, you'll do so via widget-specific commands or options,
e.g., the label's `foreground` and `background` configuration options. For most
themed widgets, color changes are specified through styles, not by changing the
widget directly.

You can specify colors via RGB, as you would in HTML or CSS, e.g. `#3FF` or
`#FF016A`. Tk also recognizes names such as `red`, `black`, `grey50`,
`light blue`, etc.

> Tk recognizes the standard names for colors defined by X11. You can find a
complete list in the command reference (noted above).

As with fonts, both macOS and Windows specify many system-specific abstract
color names (again, see the reference). The actual color these correspond to may
depend on system settings and can change over time, e.g., dark mode, text
highlight colors, default backgrounds.

If needed, you can find the RGB values (each between 0 and 65535) for a color
using the `winfo_rgb` method on any widget.

```rust,no_run
// cargo run --example colors

use tk::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    println!( "{:?}", root.winfo_rgb( TkColor::Name("red") )? );

    Ok( main_loop() )
}
```
