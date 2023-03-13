// cargo run --example paned_windows

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let p = root.add_ttk_panedwindow( -orient("vertical") )?.pack(())?;
    
    // two panes, each of which would get widgets gridded into it:
    let f1 = p.add_ttk_labelframe( -text("Pane1") -width(100) -height(100) )?;
    let f2 = p.add_ttk_labelframe( -text("Pane2") -width(100) -height(100) )?;
    p.add( &f1, () )?;
    p.add( &f2, () )?;

    Ok( main_loop() )
}
