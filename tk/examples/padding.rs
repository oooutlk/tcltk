// cargo run --example padding

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let c = root.add_ttk_frame( "c" -padding((3,3,12,12)) )?;
    c.add_ttk_frame( "f" -borderwidth(5) -relief("ridge") -width(200) -height(100) )?;
    c.add_ttk_label( "namelbl" -text("Name") )?;
    c.add_ttk_entry( "name" )?;
    c.add_ttk_checkbutton( "one"   -text("One")   -variable("one")   -onvalue(1) )?; tk.set( "one"  , 1 );
    c.add_ttk_checkbutton( "two"   -text("Two")   -variable("two")   -onvalue(1) )?; tk.set( "two"  , 0 );
    c.add_ttk_checkbutton( "three" -text("Three") -variable("three") -onvalue(1) )?; tk.set( "three", 1 );
    c.add_ttk_button( "ok" -text("Okay") )?;
    c.add_ttk_button( "cancel" -text("Cancel") )?;

    tk.grid( ".c"         -column(0) -row(0) -sticky("nsew") )?;
    tk.grid( ".c.f"       -column(0) -row(0) -columnspan(3) -rowspan(2) -sticky("nsew") )?;
    tk.grid( ".c.namelbl" -column(3) -row(0) -columnspan(2)             -sticky("nw")  -padx(5) )?;
    tk.grid( ".c.name"    -column(3) -row(1) -columnspan(2)             -sticky("new") -padx(5) -pady(5) )?;
    tk.grid( ".c.one"     -column(0) -row(3) )?;
    tk.grid( ".c.two"     -column(1) -row(3) )?;
    tk.grid( ".c.three"   -column(2) -row(3) )?;
    tk.grid( ".c.ok"      -column(3) -row(3) )?;
    tk.grid( ".c.cancel"  -column(4) -row(3) )?;

    root.grid_columnconfigure( 0, -weight(1) )?;
    root.grid_rowconfigure(    0, -weight(1) )?;
    c.grid_columnconfigure(    0, -weight(3) )?;
    c.grid_columnconfigure(    1, -weight(3) )?;
    c.grid_columnconfigure(    2, -weight(3) )?;
    c.grid_columnconfigure(    3, -weight(1) )?;
    c.grid_columnconfigure(    4, -weight(1) )?;
    c.grid_rowconfigure(       1, -weight(1) )?;

    Ok( main_loop() )
}
