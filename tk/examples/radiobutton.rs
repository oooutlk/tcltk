use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    root.add_ttk_radiobutton( "home"   -text("Home")   -variable("phone") -value("home")   )?.pack(())?;
    root.add_ttk_radiobutton( "office" -text("Office") -variable("phone") -value("office") )?.pack(())?;
    root.add_ttk_radiobutton( "cell"   -text("Mobile") -variable("phone") -value("cell")   )?.pack(())?;

    Ok( main_loop() )
}
