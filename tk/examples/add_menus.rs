// cargo run --example add_menus

use std::path::PathBuf;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    tk.option_add( "*tearOff", 0 )?;

    let win = root.add_toplevel(())?;
    let created_widgets = win.add_menus( "menubar"
        -menu::cascade( "file" -label("File")
            -menu::command( -label("New")       -command("newFile") )
            -menu::command( -label("Open...")   -command("openFile") )
            -menu::command( -label("Close")     -command("closeFile") )
            -menu::cascade( "recent" -label("Open Recent") )
            -menu::separator( no_arg() )
            -menu::checkbutton( -label("Check") -variable("check") -onvalue(1) -offvalue(0) )
            -menu::radiobutton( -label("One")   -variable("radio") -value(1) )
            -menu::radiobutton( -label("Two")   -variable("radio") -value(2) )
        )
    )?;

    let menu_recent = created_widgets.query::<TkMenu<_>>( "menubar.file.recent" ).unwrap();
    let recent_files = [
        "/some/place/lorum.txt",
        "/another/place/ipsum.md",
        "/yet_another/place/dolor.toml",
    ];
    for f in recent_files {
        let f = PathBuf::from(f);
        if let Some( file_name ) = f.file_name() {
            menu_recent.add_command(
                -label( file_name.to_str() )
                -command(( "openFile", f ))
            )?;
        }
    }
    // menu_recent.delete_range( 0.. )?;

    Ok( main_loop() )
}
