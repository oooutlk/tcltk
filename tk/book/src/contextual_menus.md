# Contextual Menus

Contextual menus ("popup" menus) are typically invoked by a right mouse button
click on an object in the application. A menu pops up at the location of the
mouse cursor. Users can then select an items from the menu (or click outside it
to dismiss it without choosing any item).

To create a contextual menu, we'll use exactly the same commands we did to
create menus in the menubar. Typically, we'd create one menu with several
command items in it, and potentially some cascade menu items and their
associated menus.

To activate the menu, users will perform a contextual menu click. We'll have to
create an event binding to capture that click. That, however, can mean different
things on different platforms. On Windows and X11, this is the right mouse
button being clicked (the third mouse button). On macOS, this is either a click
of the left (or only) button with the control key held down or a right-click on
a multi-button mouse. Unlike Windows and X11, macOS refers to this as the second
mouse button, not the third, so that's the event we'll see in our program.

> Most earlier programs that have used popup menus assumed it was only
"button 3" they needed to worry about.

Besides capturing the correct contextual menu event, we also need to capture the
mouse's location. It turns out we need to do this relative to the entire screen
(global coordinates) and not local to the window or widget you clicked on (local
coordinates). The `%X` and `%Y` substitutions in Tk's event binding system will
capture those for us.

The last step is simply to tell the menu to pop up at the particular location,
via the `post` method. Here's an example of the whole process, using a popup
menu on the application's main window.

```rust,no_run
// `cargo run --example contextual_menus`

use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let menu = root.add_menu(())?;
    for i in ["One", "Two", "Three"] {
        menu.add_command( -label(i) )?;
    }

    let handler = tclosure!( tk,
        |evt_rootx, evt_rooty| -> TkResult<()> {
            Ok( tk.popup( menu, evt_rootx, evt_rooty, None )? )
        }
    );

    use event::*;
    if tk.windowing_system()? == TkWindowingSystem::Aqua {
        root.bind( button_press_2(), &*handler )?;
        root.bind( control().button_press_1(), &*handler )?;
    } else {
        root.bind( button_press_3(), &*handler )?;
    }

    Ok( main_loop() )
}
```
