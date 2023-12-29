// cargo run --example dialog_windows

use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    tk.choose_color( -initialcolor("#ff0000") )?;

    let l = root
        .add_ttk_label( "l" -text("Hello World") -font("helvetica 24") )?
        .grid( -padx(10) -pady(10) )?;
    let on_font_changed = tclosure!( tk,
        |some_font:Obj| -> TkResult<()> {
            Ok( l.configure( -font(some_font) )? )
        }
    );
    tk.fontchooser_configure( -font("helvetica 24") -command(on_font_changed) )?;

    tk.fontchooser_show()?;
    tk.fontchooser_hide()?;

    tk.message_box( -message("Have a good day") )?;

    tk.message_box(
        -type_( "yesno" )
        -message( "Are you sure you want to install SuperVirus?" )
        -icon( "question" ) -title( "Install" )
    )?;

    Ok( main_loop() )
}
