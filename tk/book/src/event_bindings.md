# Event Bindings

We've already seen that the canvas widget as a whole, like any other Tk widget,
can capture events using the bind command.

You can also attach bindings to individual items in the canvas (or groups of
them, as we'll see in the next section using tags). So if you want to know
whether or not a particular item has been clicked on, you don't need to watch
for mouse click events for the canvas as a whole and then figure out if that
click happened on your item. Tk will take care of all this for you.

To capture these events, you use a bind command built into the canvas. It works
exactly like the regular bind command, taking an event pattern and a callback.
The only difference is you specify the canvas item this binding applies to.

```rust,no_run
canvas.bind(
    id.into(),
    event::button_press_1(),
    tclosure!( tk, move || l.configure( -text("...") )))?;
```

Let's add some code to our sketchpad example to allow changing the drawing
color. We'll first create a few different rectangle items, each filled with a
different color. We'll then attach a binding to each of these. When they're
clicked on, they'll set a global variable to the new drawing color. Our mouse
motion binding will look at that variable when creating the line segments.

```rust,no_run
// cargo run --example canvas_event_binding

use std::os::raw::c_double;
use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    let canvas = root
        .add_canvas(())?
        .grid( -sticky("nwes") -column(0i32) -row(0i32) )?;
    root.grid_columnconfigure( 0, -weight(1) )?;
    root.grid_rowconfigure( 0, -weight(1) )?;
    Widget::bind( &canvas, event::button_press_1(), "set lastx %x; set lasty %y" )?;
    Widget::bind( &canvas, event::button_1().motion(), tclosure!( tk, args: "%x %y",
        move |x: c_double, y: c_double| -> TkResult<()> {
            let last_x = tk.get_double("lastx")?;
            let last_y = tk.get_double("lasty")?;
            let color = tk.get("color")?;
            canvas.create_line( &[ (last_x,last_y), (x,y) ], -fill(color) )?;
            tk.set( "lastx", x );
            tk.set( "lasty", y );
            Ok(())
        }
    ))?;

    let id = canvas.create_rectangle( 10.0, 10.0, 30.0, 30.0, -fill("red") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, move || { tk.set( "color", "red" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 35.0, 30.0, 55.0, -fill("blue") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, move || { tk.set( "color", "blue" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 60.0, 30.0, 80.0, -fill("black") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, move || { tk.set( "color", "black" ); Ok(()) }))?;

    tk.set( "color", "black" );
    Ok( main_loop() )
}
```
