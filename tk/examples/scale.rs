// cargo run --example scale

use std::os::raw::c_double;
use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    // label tied to the same variable as the scale, so auto-updates
    root.add_ttk_label( "auto" -textvariable("num") )?
        .grid( -column(0) -row(0) -sticky("we") )?;
    
    // label that we'll manually update via the scale's command callback
    let manual = root.add_ttk_label( "manual" )?
        .grid( -column(0) -row(1) -sticky("we") )?;
    
    let scale =
        root.add_ttk_scale( "scale"
            -orient(   "horizontal" )
            -length(   "200"        )
            -from(     1.0          )
            -to(       100.0        )
            -variable( "num"        )
            -command( tkbind!( tk, |val: c_double| -> TkResult<()> {
                Ok( manual.configure( -text( format!( "Scale at {}", val )))? )
            }))
        )?
       .grid( -column(0) -row(2) -sticky("we") )?; 
    
    scale.set( 20.0 )?;

    Ok( main_loop() )
}
