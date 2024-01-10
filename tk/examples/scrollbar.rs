// cargo run --example scrollbar

use std::os::raw::c_double;

use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let l = root.add_listbox( "l" -height(5) )?
        .grid( -column(0) -row(0) -sticky("nwes") )?;

    let s = root.add_ttk_scrollbar( "s"
            -orient("vertical")
            -command( tkbind!( tk,
                |..| -> TkResult<()> { Ok( l.yview_( tcl_va_args!() )? )})))?
        .grid( -column(1) -row(0) -sticky("ns") )?;

    l.configure( -yscrollcommand( tkbind!( tk,
        |first:c_double, last:c_double| -> TkResult<()> { Ok( s.set( first, last )? )})))?;

    root.add_ttk_label( "stat" -text("Status message here") -anchor("w") )?
        .grid( -column(0) -columnspan(2) -row(1) -sticky("we") )?;

    root.grid_columnconfigure( 0, -weight(1) )?;
    root.grid_rowconfigure(    0, -weight(1) )?;

    for i in 0..100 {
       l.insert_end( Some( Obj::from( format!( "Line {} of 100", i ))))?;
    }

    Ok( main_loop() )
}
