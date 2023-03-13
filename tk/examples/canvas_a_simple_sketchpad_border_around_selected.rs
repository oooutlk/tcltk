// cargo run --example canvas_a_simple_sketchpad_border_around_selected

use std::os::raw::c_double;
use tcl::*;
use tk::*;
use tk::canvas::*;
use tk::cmd::*;

fn set_color<Inst:TkInstance>( tk: &Tk<Inst>, canvas: &TkCanvas<Inst>, color: &Obj ) -> TkResult<()> {
    tk.set( "color", color.clone() );
    canvas.dtag( item_tag( "all" ), Some( item_tag( "paletteSelected" )))?;
    canvas.itemconfigure( item_tag( "palette" ), -outline("white") )?;
    canvas.addtag( "paletteSelected", SearchSpec::WithTag( item_tag( &format!( "palette{}", color.clone() )).into() ))?;
    canvas.itemconfigure( item_tag( "paletteSelected" ), -outline("#999999") )?;
    Ok(())
}

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
            set_color( &tk, &canvas, &color )?;
            canvas.create_line( &[ (last_x,last_y), (x,y) ], -fill(color) )?;
            tk.set( "lastx", x );
            tk.set( "lasty", y );
            Ok(())
        }
    ))?;

    let id = canvas.create_rectangle( 10.0, 10.0, 30.0, 30.0,
        -fill("red") -tags("palette palettered") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, move || { tk.set( "color", "red" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 35.0, 30.0, 55.0,
        -fill("blue") -tags("palette paletteblue") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, move || { tk.set( "color", "blue" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 60.0, 30.0, 80.0,
        -fill("black") -tags("palette paletteblack paletteSelected") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, move || { tk.set( "color", "black" ); Ok(()) }))?;

    set_color( &tk, &canvas, &Obj::from("black") )?;
    canvas.itemconfigure( item_tag( "palette" ), -width(5) )?;

    Ok( main_loop() )
}
