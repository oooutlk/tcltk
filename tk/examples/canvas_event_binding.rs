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
    Widget::bind( &canvas, event::button_1().motion(), tkbind!( tk,
        |evt_x:c_double, evt_y:c_double| -> TkResult<()> {
            let last_x = tk.get_double("lastx")?;
            let last_y = tk.get_double("lasty")?;
            let color = tk.get("color")?;
            canvas.create_line( &[ (last_x,last_y), (evt_x,evt_y) ], -fill(color) )?;
            tk.set( "lastx", evt_x );
            tk.set( "lasty", evt_y );
            Ok(())
        }
    ))?;

    let id = canvas.create_rectangle( 10.0, 10.0, 30.0, 30.0, -fill("red") )?;
    canvas.bind( id,
        event::button_press_1(),
        tkbind!( tk, || { tk.set( "color", "red" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 35.0, 30.0, 55.0, -fill("blue") )?;
    canvas.bind( id,
        event::button_press_1(),
        tkbind!( tk, || { tk.set( "color", "blue" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 60.0, 30.0, 80.0, -fill("black") )?;
    canvas.bind( id,
        event::button_press_1(),
        tkbind!( tk, || { tk.set( "color", "black" ); Ok(()) }))?;

    tk.set( "color", "black" );
    Ok( main_loop() )
}
