// cargo run --example canvas_a_simple_sketchpad

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
            canvas.create_line( &[ (last_x,last_y), (x,y) ], () )?;
            tk.set( "lastx", x );
            tk.set( "lasty", y );
            Ok(())
        }
    ))?;
    Ok( main_loop() )
}
