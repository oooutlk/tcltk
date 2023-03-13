// cargo run --example creating_widgets_in_one_expression_without_geometry

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    tk.root().add_widgets(
        -label( "lbl" -text("geometry managers separated") )
        -frame( "fr" -button( "btn" -text("quit") -command("destroy .") ))
    )?;
    tk.pack( ".lbl .fr .fr.btn" )?;
    Ok( main_loop() )
}
