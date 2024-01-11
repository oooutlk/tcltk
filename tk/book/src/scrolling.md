# Scrolling

In many applications, you'll want the canvas to be larger than what appears on
the screen. You can attach horizontal and vertical scrollbars to the canvas in
the usual way via the `xview` and `yview` methods.

You can specify both how large you'd like it to be on screen and its full size
(which would require scrolling to see). The `width` and `height` configuration
options control how much space the canvas widget requests from the geometry
manager. The `scrollregion` configuration option tells Tk how large the canvas
surface is by specifying its left, top, right, and bottom coordinates, e.g.,
`0 0 1000 1000`.

You should be able to modify the sketchpad program to add scrolling, given what
you already know. Give it a try.

Once you've done that, scroll the canvas down just a little bit, and then try
drawing. You'll see that the line you're drawing appears above where the mouse
is pointing! Surprised?

What's going on is that the global `bind` command doesn't know that the canvas
is scrolled (it doesn't know the details of any particular widget). So if you've
scrolled the canvas down by 50 pixels, and you click on the top left corner,
bind will report that you've clicked at (0,0). But we know that because of the
scrolling, that position should really be (0,50).

The `canvasx` and `canvasy` methods translate the position onscreen (which bind
reports) into the actual point on the canvas (taking into account scrolling).

> Be careful if you're adding `canvasx` and `canvasy` methods directly to the
event binding scripts. You need to watch the quoting and substitutions to ensure
the conversions are done when the event fires. As always, it's better to place
that code in a procedure separate from the event binding itself.

Here then, is our complete example. We probably don't want the palette to be
scrolled away when the canvas is scrolled, but we'll leave that for another day.

```rust,no_run
// cargo run --example canvas_scrolling

use std::os::raw::c_double;
use tcl::*;
use tk::*;
use tk::canvas::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let canvas = root
        .add_canvas( "canvas" -scrollregion("0 0 1000 1000") -yscrollcommand(".v set") -xscrollcommand(".h set") )?
        .grid( -sticky("nwes") -column(0i32) -row(0i32) )?;

    root.grid_columnconfigure( 0, -weight(1) )?;
    root.grid_rowconfigure( 0, -weight(1) )?;

    let _h = root
        .add_ttk_scrollbar( "h" -orient("horizontal") -command(".canvas xview") )?
        .grid( -column(0) -row(1) -sticky("we") )?;

    let _v = root
        .add_ttk_scrollbar( "v" -orient("vertical") -command(".canvas yview") )?
        .grid( -column(1) -row(0) -sticky("ns") )?;

    Widget::bind( &canvas, event::button_press_1(),
        "set lastx [.canvas canvasx %x]; set lasty [.canvas canvasy %y]" )?;
    Widget::bind( &canvas, event::button_1().motion(), tclosure!( tk,
        |evt_x:c_double, evt_y:c_double| -> TkResult<()> {
            let x = canvas.canvasx( evt_x, None )?;
            let y = canvas.canvasy( evt_y, None )?;

            let last_x = tk.get_double("lastx")?;
            let last_y = tk.get_double("lasty")?;
            let color = tk.get("color")?;

            canvas.dtag( item_tag( "all" ),
                Some( ItemTag( "paletteSelected".to_owned() )))?;
            canvas.itemconfigure( item_tag( "palette" ),
                -outline("white") )?;
            canvas.addtag( "paletteSelected",
                SearchSpec::WithTag(
                    item_tag( &format!( "palette{}", color.clone().get_string() )).into() ))?;
            canvas.itemconfigure( item_tag( "paletteSelected" ), -outline("#999999") )?;

            canvas.create_line( &[ (last_x,last_y), (x,y) ],
                -fill(color) -width(5) -tags("currentline") )?;

            tk.set( "lastx", x );
            tk.set( "lasty", y );
            Ok(())
        }
    ))?;

    Widget::bind(
        &canvas,
        event::button_1().button_release(),
        tclosure!( tk, || ->TkResult<()> {
            Ok( canvas.itemconfigure( item_tag( "currentline" ), -width(1) )? )
        })
    )?;

    let id = canvas.create_rectangle( 10.0, 10.0, 30.0, 30.0,
        -fill("red") -tags("palette palettered") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, || { tk.set( "color", "red" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 35.0, 30.0, 55.0,
        -fill("blue") -tags("palette paletteblue") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, || { tk.set( "color", "blue" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 60.0, 30.0, 80.0,
        -fill("black") -tags("palette paletteblack paletteSelected") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, || { tk.set( "color", "black" ); Ok(()) }))?;

    tk.set( "color", "black" );
    canvas.itemconfigure( item_tag( "palette" ), -width(5) )?;

    Ok( main_loop() )
}
```
