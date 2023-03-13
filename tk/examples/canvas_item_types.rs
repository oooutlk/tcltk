use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    let canvas = root.add_canvas(())?.pack(())?;

    canvas.create_line( &[ (10.0,10.0), (200.0,50.0), (90.0,150.0), (50.0,80.0) ], () )?;
    canvas.create_rectangle( 10.0, 10.0, 200.0, 50.0, -fill("red") -outline("blue") )?;
    canvas.create_oval( 10.0, 10.0, 200.0, 50.0, -fill("red") -outline("blue") )?;
    canvas.create_polygon( &[ (10.0,10.0), (200.0,50.0), (90.0,150.0), (50.0,80.0), (120.0,55.0) ],
        -fill("red") -outline("blue") )?;
    canvas.create_arc( 10.0, 10.0, 200.0, 50.0,
        -fill("yellow") -outline("black") -start(45) -extent(135) -width(5) )?;

    let button = root.add_ttk_button( -text("Implode!") )?;
    canvas.create_window( 10.0, 10.0, -anchor("nw") -window(button) )?;

    Ok( main_loop() )
}
