// `cargo run --example contextual_menus`

use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let menu = root.add_menu(())?;
    for i in ["One", "Two", "Three"] {
        menu.add_command( -label(i) )?;
    }

    let handler = tkbind!( tk,
        |evt_rootx, evt_rooty| -> TkResult<()> {
            Ok( tk.popup( menu, evt_rootx, evt_rooty, None )? )
        }
    );

    use event::*;
    if tk.windowing_system()? == TkWindowingSystem::Aqua {
        root.bind( button_press_2(), &*handler )?;
        root.bind( control().button_press_1(), &*handler )?;
    } else {
        root.bind( button_press_3(), &*handler )?;
    }

    Ok( main_loop() )
}
