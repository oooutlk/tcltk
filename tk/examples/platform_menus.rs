use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let win = root.add_toplevel("win")?;
    let menubar = win.add_menu("menubar")?;
    let apple = menubar.add_menu("apple")?;
    menubar.add_cascade( -menu(apple) )?;
    apple.add_command( -label("About My Application") )?;
    apple.add_separator(())?;
    win.configure( -menu(menubar) )?;

    tclosure!( tk, cmd: "tk::mac::ShowPreferences", move || -> TkResult<()> {
        // show preferences
        Ok(())
    });

    #[cfg( target_os = "macos" )]
    {
        let menu_help = menubar.add_menu( "help" )?;
        menubar.add_cascade( -menu(menu_help) -label("Help") )?;
        tclosure!( tk, cmd: "::tk::mac::ShowHelp", move || -> TkResult<()> {
            //show help
            Ok(())
        });
    }

    #[cfg( not( target_os = "macos" ))]
    {
        #[cfg( windows )]
        menubar.add_cascade( -menu( menubar.add_menu( "system" )? ))?;

        #[cfg( unix )]
        menubar.add_cascade( -label("Help") -menu( menubar.add_menu("help")? ))?;
    }
    Ok( main_loop() )
}
