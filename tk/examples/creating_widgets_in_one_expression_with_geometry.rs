// cargo run --example creating_widgets_in_one_expression_with_geometry

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    tk.root().add_widgets(
        -pack( -label( -text("all in one") ))
        -pack( -frame( -pack( -button( "btn" -text("quit") -command("destroy .") ))))
    )?;
    Ok( main_loop() )
}
