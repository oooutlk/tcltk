use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    let canvas = root.add_canvas(())?.pack(())?;

    // To create a line, you need to specify its starting and ending coordinates.
    // Coordinates are expressed as the number of pixels away from the top-left corner,
    // horizontally and vertically, i.e. (x,y). The pixel at the top-left corner, known
    // as the origin, has coordinates (0,0). The "x" value increases as you move to the
    // right, and the "y" value increases as you move down. A line is described by two
    // points, which we'd refer to as (x1,y1) and (x2,y2). This code creates a line
    // from (10,5) to (200,50):
    canvas.create_line( &[ (10.0,5.0), (200.0,50.0) ], () )?;

    // When creating items, you can also specify one or more item attributes, affecting
    // how it appears. For example, we can specify that the line should be red and
    // three pixels wide.
    canvas.create_line( &[ (10.0,10.0), (200.0,50.0) ], -fill("red")-width(3) )?;

    // Just like with Tk widgets, you can change the attributes of canvas items after
    let id = canvas.create_line( &[ (0.0,0.0), (10.0,10.0) ], -fill("red") )?;
    canvas.itemconfigure( id, -fill("blue") -width(2) )?;

    Ok( main_loop() )
}
